`timescale 1ns / 1ps

/// SpiMasterPeripheral
/// A memory-mapped SPI Master peripheral module.
/// This module wraps the SpiMaster module and provides a memory-mapped
/// interface for controlling SPI transfers and accessing received data.
module SpiMasterPeripheral #(
    parameter CPOL = 0,
    parameter CPHA = 0
) (
    // Reset and clock
    input wire clk,
    input wire rst,

    // rclk -- Reference clock (used to derive SPI clock -- see SpiMaster module)
    input wire rclk,
    
    // SPI interface
    output wire spi_clk,
    output wire spi_mosi,
    input wire spi_miso,
    
    // Peripheral interface - Memory Mapped I/O
    // mem_addr - Address of the register to access (8-bit address space)
    // mem_wr_en - Write enable signals (4 bits for byte enables - bit mask for 4 bytes)
    // mem_wr_data - Data to write to the register (32 bits)
    // mem_rd_data - Data read from the register (32 bits)
    // memdata is little-endian
    input wire [7:0] mem_addr,
    input wire [3:0] mem_wr_en,
    input wire [31:0] mem_wr_data,
    output wire [31:0] mem_rd_data,

    // Debug interface
    output wire debug_busy,
    output wire debug_ready,
    output wire debug_tx,
    output wire debug_rx,
    output wire debug_start
);

    // Internal signals (clock-domain: rclk)
    reg driver_start = 1'b0;
    reg driver_reset = 1'b0;
    reg [7:0] driver_tx_data = 8'b0;
    wire [7:0] driver_rx_data;
    wire driver_ready;
    wire driver_busy;

    // Internal signals (clock-domain: clk)
    reg start = 1'b0;
    reg soft_reset = 1'b0;
    wire [7:0] tx_data;
    wire [7:0] rx_data;
    reg ready = 1'b0;
    reg busy = 1'b0;
    wire mmio_start; // start signal - one clk pulse to trigger transfer
    wire mmio_soft_reset; // reset signal - one clk pulse to trigger reset

    // SpiMaster -- The actual SPI driver
    SpiMaster #(
        .CPOL(CPOL),
        .CPHA(CPHA),
    ) spi_master_inst (
        .rst(driver_reset),
        .rclk(rclk),
        .spi_clk(spi_clk),
        .spi_mosi(spi_mosi),
        .spi_miso(spi_miso),
        .start(driver_start),
        .tx_data(tx_data),
        .rx_data(rx_data),
        .busy(driver_busy),
        .ready(driver_ready)
    );

    // SpiMasterInnerPeripheral -- Memory-mapped I/O interface
    SpiMasterInnerPeripheral spi_master_mmio_inst (
        .clk(clk),
        .rst(rst),
        .mem_addr(mem_addr),
        .mem_wr_en(mem_wr_en),
        .mem_wr_data(mem_wr_data),
        .mem_rd_data(mem_rd_data),
        .start(mmio_start),
        .soft_reset(mmio_soft_reset),
        .busy(busy),
        .ready(ready),
        .rx_data(rx_data),
        .tx_data(tx_data)
    );

    // The funny part is there - Clock domain crossing between clk and rclk
    // We use simple handshaking for start and driver_reset signals

    // Clock domain crossing for start/reset signal
    reg [1:0] start_rclk_sync; // 2-stage synchronizer for start (0 - meta, 1 - sync)
    reg [1:0] reset_rclk_sync; // 2-stage synchronizer for reset (0 - meta, 1 - sync)
    always @(posedge rclk or posedge rst) begin
        if (rst) begin
            start_rclk_sync <= 2'b0;
            driver_start <= 1'b0;
            reset_rclk_sync <= 2'b0;
            driver_reset <= 1'b0;
        end else begin
            start_rclk_sync[0] <= start;
            start_rclk_sync[1] <= start_rclk_sync[0];
            driver_start <= start_rclk_sync[1];
            reset_rclk_sync[0] <= soft_reset;
            reset_rclk_sync[1] <= reset_rclk_sync[0];
            driver_reset <= reset_rclk_sync[1];
        end
    end

    // Clock domain crossing for start/reset ack signals
    reg [1:0] start_clk_ack;   // 2-stage ack synchronizer for start (0 - meta, 1 - sync)
    reg [1:0] reset_clk_ack;   // 2-stage ack synchronizer for reset (0 - meta, 1 - sync)
    always @(posedge clk or posedge rst) begin
        if (rst) begin
            start_clk_ack <= 2'b0;
            start <= 1'b0;
            reset_clk_ack <= 2'b0;
            soft_reset <= 1'b0;
        end else begin
            reset_clk_ack[0] <= driver_reset;
            reset_clk_ack[1] <= reset_clk_ack[0];
            start_clk_ack[0] <= driver_start;
            start_clk_ack[1] <= start_clk_ack[0];

            // Start signal 
            // Must be reset when acknowledged
            // Must be set high when mmio_start is triggered
            start <= mmio_start ? 1'b1 : start_clk_ack[1] ? 1'b0 : start;

            // Soft reset signal
            // Must be reset when acknowledged
            // Must be set high when mmio_soft_reset is triggered
            soft_reset <= mmio_soft_reset ? 1'b1 : reset_clk_ack[1] ? 1'b0 : soft_reset;
        end
    end

    // Clock domain crossing for rx_data, ready, busy signals
    reg [1:0] ready_clk_sync;        // 2-stage synchronizer for ready
    reg [1:0] busy_clk_sync;         // 2-stage synchronizer for busy
    always @(posedge clk or posedge rst) begin
        if (rst) begin
            ready_clk_sync <= 2'b0;
            busy_clk_sync <= 2'b0;
        end else begin
            ready_clk_sync[0] <= driver_ready;
            ready_clk_sync[1] <= ready_clk_sync[0];
            ready <= ready_clk_sync[1];
            busy_clk_sync[0] <= driver_busy;
            busy_clk_sync[1] <= busy_clk_sync[0];
            busy <= busy_clk_sync[1];
        end
    end

    // Debug signals
    assign debug_busy = busy;
    assign debug_ready = ready;
    assign debug_tx = tx_data;
    assign debug_rx = rx_data;
    assign debug_start = mmio_start;
endmodule

/// SpiMasterInnerPeripheral
/// This module is not supposed to be used directly. It is there to split up
/// the actual SpiMasterPeripheral between the actual mmap-io part and the 
/// (clock-)domain-crossing part.
module SpiMasterInnerPeripheral (
    // Reset and clock
    // clk - Peripheral clock domain (for memory-mapped I/O)
    input wire clk,
    // rst - Peripheral reset signal
    input wire rst,
    
    // Peripheral interface - Memory Mapped I/O
    // mem_addr - Address of the register to access (8-bit address space)
    // mem_wr_en - Write enable signals (4 bits for byte enables - bit mask for 4 bytes)
    // mem_wr_data - Data to write to the register (32 bits)
    // mem_rd_data - Data read from the register (32 bits)
    // memdata is little-endian
    input wire [7:0] mem_addr,
    input wire [3:0] mem_wr_en,
    input wire [31:0] mem_wr_data,
    output wire [31:0] mem_rd_data,

    // Trigger and data signals
    output reg start,
    output reg soft_reset,
    input wire busy,
    input wire ready,
    input wire [7:0] rx_data,
    output reg [7:0] tx_data
);
    // -- Peripheral registers --
    // REG_CONTROL: Control register
    //   Address offset: 0x00
    //   Write: Bit 0 = start (trigger transfer), Bit 1 = reset (reset internal state)
    //   Read: N/A (returns 0)
    localparam REG_CONTROL = 10'h00;
    // REG_WRITE_DATA: Write-only register to write data to be sent over SPI
    //   Address offset: 0x04
    //   Write: Writing a byte to this register triggers an SPI transfer.
    //   Read: N/A (returns 0)
    localparam REG_WRITE_DATA = 10'h04;
    // REG_READ_DATA: Read-only register to read data received from SPI
    //   Address offset: 0x08
    //   Write: N/A (ignored)
    //   Read: Reading from this register returns the last byte received from SPI.
    localparam REG_READ_DATA = 10'h08;
    // REG_STATUS: Read-only register to read status of the SPI master
    //   Address offset: 0x0C
    //   Write: N/A (ignored)
    //   Read: Bit 0 = ready, Bit 1 = busy, Bit 2 = tx_busy
    localparam REG_STATUS = 10'h0C;
    // REG_READ_AND_STATUS: Read-only register to read both data and status
    //   Address offset: 0x10
    //   Write: N/A (ignored)
    //   Read: Bits [7:0] = rx_data, Bit 8 = ready, Bit 9 = busy, Bit 10 = tx_busy
    localparam REG_READ_AND_STATUS = 10'h10;
    // -- End of Peripheral registers --

    // Helper signals 
    wire [9:0] mem_addr_ext = {mem_addr, 2'b00}; // Extend to word-aligned address

    // Handle Reset and Write to memory-mapped I/O
    always @(posedge clk or posedge rst) begin
        if (rst) begin
            start <= 1'b0;
            tx_data <= 8'b0;
            soft_reset <= 1'b0;
        end else begin
            // Handle writes
            if (|mem_wr_en) begin
                start <=    (mem_addr_ext == REG_CONTROL && mem_wr_en[0] && mem_wr_data[0]) ? 1'b1 : // CONTROL - start bit
                            (mem_addr_ext == REG_WRITE_DATA && mem_wr_en[0]) ? 1'b1 : // WRITE_DATA - start transfer on write
                            1'b0; // Clear start otherwise
                soft_reset <= (mem_addr_ext == REG_CONTROL && mem_wr_en[0] && mem_wr_data[1]) ? 1'b1 : 1'b0; // CONTROL - reset bit
                if (mem_addr_ext == REG_WRITE_DATA && mem_wr_en[0]) begin
                    tx_data <= mem_wr_data[7:0];
                end
            end else begin
                start <= 1'b0; // Clear start if no write
                soft_reset <= 1'b0; // Clear reset if no write
            end
        end
    end

    // Handle Read from memory-mapped I/O (must be combinational)
    assign mem_rd_data = rst ? 32'b0 :
                         (mem_addr_ext == REG_WRITE_DATA || mem_addr_ext == REG_CONTROL) ? 32'b0 :
                         (mem_addr_ext == REG_READ_DATA) ? {24'b0, rx_data} :
                         (mem_addr_ext == REG_STATUS) ? {30'b0, busy, ready} :
                         (mem_addr_ext == REG_READ_AND_STATUS) ? {22'b0, busy, ready, rx_data} :
                         32'b0; // Default case
endmodule