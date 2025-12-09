`timescale 1ns / 1ps
// Testbench for SpiMaster module
module SpiMaster_tb ();
    // Parameters
    parameter CLK_PERIOD = 20; // Clock period for clk
    parameter ICLK_PERIOD = 5; // Clock period for iclk
    parameter CPOL = 0;
    parameter CPHA = 0;
    parameter CLK_DIVIDER = 4;

    // Testbench signals
    reg clk;
    reg rst;
    reg iclk;
    reg start;
    reg [7:0] tx_data;
    wire [7:0] rx_data;
    wire completed;
    wire spi_clk;
    wire spi_mosi;
    reg spi_miso;

    // The Device Under Test (DUT)
    SpiMaster #(
        .CPOL(CPOL),
        .CPHA(CPHA)
    ) dut (
        .clk(clk),
        .rst(rst),
        .iclk(iclk),
        .spi_clk(spi_clk),
        .spi_mosi(spi_mosi),
        .spi_miso(spi_miso),
        .start(start),
        .tx_data(tx_data),
        .rx_data(rx_data),
        .completed(completed)
    );

    // Clock generation
    initial begin
        clk = 0;
        forever #(CLK_PERIOD/2) clk = ~clk;
    end
    initial begin
        iclk = 0;
        forever #(ICLK_PERIOD/2) iclk = ~iclk;
    end

    // Test sequence
    initial begin
        // Initialize waveform dump
        $dumpfile("./SpiMaster.vcd");
        $dumpvars(-1, SpiMaster_tb);
        $dumpon();

        // Initialize signals
        rst = 1;
        start = 0;
        tx_data = 8'h00;
        spi_miso = 1'b1;
        #(2*CLK_PERIOD);

        rst = 0;
        #(2*CLK_PERIOD);

        // Test case: Send a byte
        tx_data = 8'b10101010; // Example data to send
        start = 1;
        #(CLK_PERIOD);
        start = 0;

        // Wait for completion (8 bits = 8 SPI_clk cycles = 32 iclk cycles)
        #(32*ICLK_PERIOD);
        
        // Check received data (for this testbench, we can just print it)
        $display("Received data: %h", rx_data);
        #(2*CLK_PERIOD);
        $dumpoff();
        $finish;
    end
endmodule