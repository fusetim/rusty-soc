`timescale 1ns / 1ps
// Testbench for SpiMaster module
module SpiMaster_tb ();
    // Parameters
    parameter CLK_PERIOD = 20; // Clock period for clk
    parameter rclk_PERIOD = 5; // Clock period for rclk
    parameter CPOL = 0;
    parameter CPHA = 0;
    parameter CLK_DIVIDER = 4;

    // Testbench signals
    reg clk;
    reg rst;
    reg rclk;
    reg start;
    reg [7:0] tx_data;
    wire [7:0] rx_data;
    wire ready;
    wire busy;
    wire spi_clk;
    wire spi_mosi;
    reg spi_miso;

    // The Device Under Test (DUT)
    SpiMaster #(
        //.CPOL(CPOL),
        //.CPHA(CPHA)
    ) dut (
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
        $dumpfile("./SpiMaster_synth.vcd");
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

        // Wait for completion (8 bits = 8 SPI_clk cycles = 32 rclk cycles)
        #(32*rclk_PERIOD);
        
        // Check received data
        $display("Received data: 0x%h (expect 0xff)", rx_data);
        if(rx_data != 8'hFF) $display("Test failed: Received data does not match expected value.");
        #(2*CLK_PERIOD);

        // Test case 2: Send 0xAB and receive b10101010
        tx_data = 8'hAB;
        spi_miso = 1'b0;
        #(CLK_PERIOD);
        start = 1;

        wait (spi_clk == CPHA);
        spi_miso = 1'b1;
        wait (spi_clk != CPHA); wait (spi_clk == CPHA);
        spi_miso = 1'b0;
        wait (spi_clk != CPHA); wait (spi_clk == CPHA);
        spi_miso = 1'b1;
        start = 0;
        wait (spi_clk != CPHA); wait (spi_clk == CPHA);
        spi_miso = 1'b0;
        wait (spi_clk != CPHA); wait (spi_clk == CPHA);
        spi_miso = 1'b1;
        wait (spi_clk != CPHA); wait (spi_clk == CPHA);
        spi_miso = 1'b0;
        wait (spi_clk != CPHA); wait (spi_clk == CPHA);        
        spi_miso = 1'b1;
        wait (spi_clk != CPHA); wait (spi_clk == CPHA);        
        spi_miso = 1'b0;
        wait (ready == 1'b1);

        // Check received data
        $display("Received data: 0x%h (expect 0xaa)", rx_data);
        if(rx_data != 8'hAA) $display("Test failed: Received data does not match expected value.");

        #(2*CLK_PERIOD);
        $dumpoff();
        $finish;
    end
endmodule