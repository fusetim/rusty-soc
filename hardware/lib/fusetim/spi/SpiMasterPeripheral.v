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
    output wire [31:0] mem_rd_data
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

    // Internal signals
    reg start;
    reg soft_reset;
    reg [7:0] tx_data;
    wire [7:0] rx_data;
    wire ready;
    wire busy;
    wire [9:0] mem_addr_ext = {mem_addr, 2'b00}; // Extend to word-aligned address

    SpiMaster #(
        .CPOL(CPOL),
        .CPHA(CPHA)
    ) spi_master_inst (
        .rst(rst),
        .rclk(rclk),
        .spi_clk(spi_clk),
        .spi_mosi(spi_mosi),
        .spi_miso(spi_miso),
        .start(start),
        .tx_data(tx_data),
        .rx_data(rx_data),
        .busy(busy),
        .ready(ready)
    );

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
                            1'b0;
                soft_reset <= (mem_addr_ext == REG_CONTROL && mem_wr_en[0] && mem_wr_data[1]) ? 1'b1 : 1'b0; // CONTROL - reset bit
                if (mem_addr_ext == REG_WRITE_DATA && mem_wr_en[0]) begin
                    tx_data <= mem_wr_data[7:0];
                end
            end else begin
                start <= 1'b0; // Clear start after one cycle
                soft_reset <= 1'b0; // Clear soft reset after one cycle
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