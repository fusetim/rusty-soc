/// ULX3S TB
/// This module provides the minimal inputs/outputs needed
// to run a synthesized testbench on the ULX3S board.
`timescale 1ns / 1ns

module ulx3s_tb();
    // Internal clock
    reg clk_25mhz = 1'b0;
    always #20 clk_25mhz = ~clk_25mhz; // 25MHz clock

    // Buttons
    reg [6:0] btns = 7'b0000000;

    // LEDs
    wire [7:0] leds;

    // Oled SPI interface
    wire oled_clk;
    wire oled_mosi;
    wire oled_dc;
    wire oled_resn;
    wire oled_csn;

    // SD SPI interface
    wire sd_clk;
    wire sd_mosi;
    wire sd_miso;
    wire sd_csn;

    // Audio
    wire [3:0] audio_l;
    wire [3:0] audio_r;

    // Instantiate the top module
    top uut (
        .clk_25mhz(clk_25mhz),
        .btns(btns),
        .leds(leds),
        .oled_clk(oled_clk),
        .oled_mosi(oled_mosi),
        .oled_dc(oled_dc),
        .oled_resn(oled_resn),
        .oled_csn(oled_csn),
        .sd_clk(sd_clk),
        .sd_mosi(sd_mosi),
        .sd_miso(sd_miso),
        .sd_csn(sd_csn),
        .audio_l(audio_l),
        .audio_r(audio_r)
    );

    initial begin
        // Init waveform dump
        $dumpfile("ulx3s_tb.fst");
        $dumpvars(0, leds);
        $dumpvars(1, btns);
        //$dumpvars(2, clk_25mhz);
        $dumpvars(3, oled_clk);
        $dumpvars(4, oled_mosi);
        $dumpvars(5, oled_dc);
        $dumpvars(6, oled_resn);
        $dumpvars(7, oled_csn);
        $dumpvars(8, sd_clk);
        $dumpvars(9, sd_mosi);
        $dumpvars(10, sd_miso);
        $dumpvars(11, sd_csn);
        $dumpvars(12, audio_l);
        $dumpvars(13, audio_r);
    end

    // Display LED states
    always @(leds) begin
        $display("LEDs: %b", leds);
    end
endmodule

