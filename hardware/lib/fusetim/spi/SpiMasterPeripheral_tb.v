`timescale 1ns / 1ps
// Testbench for SpiMaster module
module SpiMasterPeripheral_tb ();
    // Parameters
    parameter CLK_PERIOD = 40; // clk - 25MHz = 40ns
    parameter rclk_PERIOD = 200; // rclk - 5MHz = 200ns
    parameter CPOL = 0;
    parameter CPHA = 0;

    // Testbench signals
    reg clk;
    reg rst;
    reg rclk;
    wire spi_clk;
    wire spi_mosi;
    reg spi_miso = 0;
    reg [9:0] mem_addr_ext;
    wire [7:0] mem_addr = mem_addr_ext[9:2];
    reg [3:0] mem_wr_en;
    reg [31:0] mem_wr_data;
    wire [31:0] mem_rd_data;

    // The Device Under Test (DUT)
    SpiMasterPeripheral #(
        //.CPOL(CPOL),
        //.CPHA(CPHA)
    ) dut (
        .clk(clk),
        .rst(rst),

        // rclk -- Reference clock (used to derive SPI clock -- see SpiMaster module)
        .rclk(rclk),

        // SPI interface
        .spi_clk(spi_clk),
        .spi_mosi(spi_mosi),
        .spi_miso(spi_miso),

        // Peripheral interface - Memory Mapped I/O
        .mem_addr(mem_addr),
        .mem_wr_en(mem_wr_en),
        .mem_wr_data(mem_wr_data),
        .mem_rd_data(mem_rd_data)
    );

    // Clock generation
    initial begin
        clk = 0;
        forever #(CLK_PERIOD/2) clk = ~clk;
    end
    initial begin
        rclk = 0;
        forever #(rclk_PERIOD/2) rclk = ~rclk;
    end

    // Test sequence
    initial begin
        // Initialize waveform dump
        $dumpfile("./SpiMasterPeripheral.vcd");
        $dumpvars(-1, SpiMasterPeripheral_tb);
        $dumpon();

        // Initialize signals
        rst = 1;
        mem_addr_ext = 10'h00;
        mem_wr_en = 0;
        mem_wr_data = 32'h00000000;
        spi_miso = 1'b0;

        // Release reset
        #(2*CLK_PERIOD);
        rst = 0;

        // SPI Initialize (need a reset first)
        mem_addr_ext = 10'h00; // Control Register
        mem_wr_en = 4'b0001; // Write enable - byte 0
        mem_wr_data = 32'h00000002; // Set the reset bit
        #(4*CLK_PERIOD); // Wait the time of a store instruction - 4 cycles
        mem_wr_en = 0; // De-assert write enable

        #(8*4*CLK_PERIOD); // Wait some time - Reset takes a few clk cycles to propagate

        // New transfer - Write 0xCA over SPI
        mem_addr_ext = 10'h04; // Write Data Register
        mem_wr_en = 4'b0001; // Write enable - byte 0
        mem_wr_data = 32'h000000CA; // Data to write
        #(4*CLK_PERIOD); // Wait the time of a store instruction - 4 cycles
        mem_wr_en = 0; // De-assert write enable
        mem_addr_ext = 10'h0C; // Status Register

        #(128*CLK_PERIOD); // Wait for transfer to complete

        // Finish simulation
        #(2*CLK_PERIOD);
        $dumpoff();
        $finish;
    end
endmodule