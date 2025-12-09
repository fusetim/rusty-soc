// SpiMaster is a module implementing a Master device for SPI communication.
// It only perform "byte-transfer" one at a time. But since, SPI_CS is 
// not managed by this module, multiple bytes are able to be sent in one transmission
// by just not disenabling the line.
module SpiMaster #(
    parameter CPOL = 0,
    parameter CPHA = 0
)
(
    // clk -- reference clock for communication with the SoC
    input wire clk,
    // rst -- active high reset
    input wire rst,
    // iclk -- internal module clock 
    // SPI clock is derived from iclk (divided down at least by two)
    input wire iclk,

    // SPI interface
    output reg spi_clk, // SPI clock
    output reg spi_mosi, // SPI Master Out Slave In
    input wire spi_miso,  // SPI Master In Slave Out
    // spi_cs -- chip select is not managed by this module, to allow multi-byte transfer.

    // SoC interface
    // start - High for one clk cycle, it starts the transfer.
    //         At that point, tx_data must already be ready.
    input wire start, 
    // tx_data - the byte to be sent
    input wire [7:0] tx_data,
    // rx_data - the received byte from the slave
    output reg [7:0] rx_data,
    // completed - High as soon as the rx_data has been received.
    //             Reset on start/rst rising edge
    output reg completed
);

reg transfer = 0;
reg enable_clk = 0;
reg [3:0] bitcnt = 0;
reg [7:0] tx_byte = 0;

reg send_clk = 1;
reg recv_clk = 0;

always @(posedge clk) begin
    if (rst) begin
        completed <= 0;
        rx_data <= 0;
        spi_clk <= 0;
        transfer <= 0;
    end else begin 
        if (start) begin
            transfer <= 1;
            bitcnt <= 0;
            if (start & ~transfer) begin
                tx_byte <= tx_data;
                enable_clk <= 1;
            end
        end 
    end
end

always @(posedge iclk) begin 
    if (rst) begin 
        enable_clk <= 0;
        send_clk <= 1;
        recv_clk <= 0;
        tx_byte <= 0;
        spi_clk <= 0;
    end else if (enable_clk) begin 
        spi_clk <= ~spi_clk;
        send_clk <= send_clk + 1;
        recv_clk <= recv_clk + 1;
    end else begin 
        spi_clk <= 0;
    end
end

always @(posedge send_clk) begin 
    bitcnt <= bitcnt + 1;
    tx_byte <= {1'b0, tx_byte[7:1]};
end

always @(posedge recv_clk) begin 
    rx_data <= {rx_data[6:0], spi_miso};
    if (bitcnt == 3'h6) begin
        completed <= 1;
    end
end

assign spi_mosi = tx_byte[0];

endmodule