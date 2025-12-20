`timescale 1ns / 1ps
// Flash-able testbench for SpiMasterPeripheral
// This testbench can be sucessfully synthesized and loaded onto hardware.
module top(
    input clk_25mhz,
    input [6:0] btns,
    output [7:0] leds,
    // OLED SPI interface
    output oled_clk,
    output oled_mosi,
    output oled_dc,
    output oled_resn,
    output oled_csn,
    // SD SPI interface
    output sd_clk,
    output sd_mosi,
    input sd_miso,
    output sd_csn,
    // Audio
    input [3:0] audio_l,
    input [3:0] audio_r
);

    // Parameters
    parameter RCLK_DIV = 25; // rclk - 100kHz = 25MHz / 250

    wire rst = btns[1];
    
    // Clock division to generate rclk from clk_25mhz
    wire rclk;
    clk_div#(
        .DIVISOR(RCLK_DIV)
    ) rclk_divider (
        .clk_in(clk_25mhz),
        .rst(rst),
        .clk_out(rclk)
    );

    // SPI signals
    wire debug_busy;
    wire debug_ready;
    wire debug_start;
    SpiMasterPeripheral_flash_tb spi_tb (
        .clk(clk_25mhz),
        .rst(rst),
        .rclk(rclk),
        .spi_clk(oled_clk),
        .spi_mosi(oled_mosi),
        .spi_miso(1'b0), // Tie MISO to 0 for testing
        .spi_cs(oled_csn),
        // Debug signals
        .debug_busy(debug_busy),
        .debug_ready(debug_ready),
        .debug_start(debug_start)
    );
    assign oled_resn = debug_busy;
    assign oled_dc = debug_start;

    // Simple LED indicator for reset
    assign leds[0] = rst ? 1'b1 : 1'b0;
    assign leds[1] = debug_busy;
    assign leds[2] = debug_ready;
    assign leds[3] = debug_start;
    assign leds[7:4] = 4'b0000;
endmodule

module clk_div#(
    parameter DIVISOR = 5
)(
    input wire clk_in,
    input wire rst,
    output reg clk_out
);
    reg [$clog2(DIVISOR)-1:0] counter = 0;

    always @(posedge clk_in or posedge rst) begin
        if (rst) begin
            counter <= 0;
            clk_out <= 0;
        end else begin
            if (counter == (DIVISOR/2 - 1)) begin
                clk_out <= ~clk_out;
                counter <= 0;
            end else begin
                counter <= counter + 1;
            end
        end
    end
endmodule

module SpiMasterPeripheral_flash_tb (
    input wire clk,
    input wire rst,
    input wire rclk,
    output wire spi_clk,
    output wire spi_mosi,
    output reg spi_cs,
    input reg spi_miso,
    // Debug signals
    output wire debug_busy,
    output wire debug_ready,
    output wire debug_start
);
    // Parameters
    parameter CPOL = 0;
    parameter CPHA = 0;

    // Testbench signals
    reg [9:0] mem_addr_ext;
    wire [7:0] mem_addr = mem_addr_ext[9:2];
    reg [3:0] mem_wr_en;
    reg [31:0] mem_wr_data;
    wire [31:0] mem_rd_data;
    reg dut_rst = 1;

    // The Device Under Test (DUT)
    SpiMasterPeripheral #(
        //.CPOL(CPOL),
        //.CPHA(CPHA)
    ) dut (
        .clk(clk),
        .rst(dut_rst),

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
        .mem_rd_data(mem_rd_data),
        // Debug interface
        .debug_busy(debug_busy),
        .debug_ready(debug_ready),
        .debug_start(debug_start)
    );

    // Test sequence
    reg [7:0] seq = 0;
    always @(posedge clk or posedge rst) begin
        if (rst) begin
            dut_rst <= 1;
            mem_addr_ext <= 10'h00;
            mem_wr_en <= 4'b0000;
            mem_wr_data <= 32'h00000000;
            seq <= 0;
            spi_miso <= 0;
            spi_cs <= 1'b1;
        end else begin
            if (seq == 0) begin
                // Release reset
                dut_rst <= 0;
                seq <= seq + 1;
                spi_cs <= 1'b1;
            end else if (seq >= 1 && seq <= 4) begin
                // Init SPI device - send RST
                spi_cs <= 1'b0;
                mem_addr_ext <= 10'h00; // Address 0x00 - control register
                mem_wr_en <= 4'b0001; // Write enable for byte 0
                mem_wr_data <= 32'h00000002; // Write 0x02 to control register to issue reset
                seq <= seq + 1;
            end else if (seq >= 5 && seq <= 30) begin
                // Clear write enable
                mem_wr_en <= 4'b0000;
                seq <= seq + 1;
            end else if (seq >= 31 && seq <= 34) begin
                // Make a transfer - send a byte 0b10010101
                mem_addr_ext <= 10'h04; // Address 0x04 - TX register
                mem_wr_en <= 4'b0001; // Write enable for byte 0
                mem_wr_data <= 32'h00000095; // Write 0x95 to TX register
                seq <= seq + 1;
            end else if (seq >= 35 && seq <= 39) begin
                // Clear write enable
                mem_wr_en <= 4'b0000;
                seq <= seq + 1;
            end else if (seq == 40) begin
                // Read status
                mem_addr_ext <= 10'h0C; // Address 0x0C - Status register
                mem_wr_en <= 4'b0000; // Write enable for byte 0
                mem_wr_data <= 32'h00000000; // Write 0x00 to Status register
                // Wait for ready
                if (debug_ready) begin
                    seq <= seq + 1;
                end
            end else if (seq >= 41 && seq <= 44) begin
                // Make a transfer - send a byte 0b00111010
                mem_addr_ext <= 10'h04; // Address 0x04 - TX register
                mem_wr_en <= 4'b0001; // Write enable for byte 0
                mem_wr_data <= 32'h0000003A; // Write 0x3A to TX register
                seq <= seq + 1;
            end else if (seq >= 45 && seq <= 49) begin
                // Clear write enable
                mem_wr_en <= 4'b0000;
                seq <= seq + 1;
            end else begin
                seq <= seq; // Hold state
            end
        end
    end

endmodule