// SpiMaster is a module implementing a Master device for SPI communication.
// It only perform "byte-transfer" one at a time. But since, SPI_CS is 
// not managed by this module, multiple bytes are able to be sent in one transmission
// by just not disenabling the line.
module SpiMaster #(
    parameter CPOL = 0,
    parameter CPHA = 0
)
(
    // rst -- active high reset
    input wire rst,
    // rclk -- Reference clock 
    // SPI clock is derived from rclk (SPI_CLK = rclk / 2)
    input wire rclk,

    // SPI interface
    output wire spi_clk, // SPI clock
    output wire spi_mosi, // SPI Master Out Slave In
    input wire spi_miso,  // SPI Master In Slave Out
    // spi_cs -- chip select is not managed by this module, to allow multi-byte transfer.

    // SoC interface
    // start - High to trigger the transfer.
    //         At that point, tx_data must already be ready.
    //         Note: This signal is only sampled on the rising edge of rclk.
    //               If start is asserted when busy, it will be ignored until the
    //               current transfer is complete.
    input wire start, 
    // tx_data - the byte to be sent
    //           This signal must be valid when start is asserted.
    //           Once busy is high, tx_data is no longer used by the current transfer.
    //           Therefore, the next byte can be safely loaded for the next transfer.
    input wire [7:0] tx_data,
    // rx_data - the received byte from the slave
    //           This signal is valid when ready is high.
    output reg [7:0] rx_data,
    // ready - High as soon as the rx_data has been received.
    //         Reset after a new transfer is started.
    output reg ready,
    // busy - High while a transfer is ongoing on the SPI bus.
    output wire busy
);
    // Internal buffer
    reg [7:0] tx_buffer;

    // State encoding
    localparam STATE_IDLE = 2'b00;
    localparam STATE_TRANSFER_WRITE = 2'b10;
    localparam STATE_TRANSFER_READ = 2'b11;

    // Internal state register
    reg [1:0] state;
    // Bit counter (number of bits already transferred)
    reg [2:0] bit_cnt;

    // State machine
    always @(posedge rclk or posedge rst) begin
        if (rst) begin
            state <= STATE_IDLE;
            bit_cnt <= 3'b0;
            ready <= 1'b0;
            rx_data <= 8'b0;
            tx_buffer <= 8'b0;
        end else begin
            case (state)
                STATE_IDLE: begin
                    if (start) begin
                        // Load the tx_buffer and start transfer
                        tx_buffer <= tx_data;
                        rx_data <= 8'b0;
                        bit_cnt <= 3'b0;
                        ready <= 1'b0;
                        state <= (CPHA == 0) ? STATE_TRANSFER_WRITE : STATE_TRANSFER_READ;
                    end
                end
                STATE_TRANSFER_WRITE: begin
                    // Shift out data on MOSI
                    state <= STATE_TRANSFER_READ;
                    if (CPHA == 1) begin
                        // Shift out next bit
                        tx_buffer <= {tx_buffer[6:0], 1'b0};
                    end
                end
                STATE_TRANSFER_READ: begin
                    // Shift in data from MISO
                    rx_data <= {rx_data[6:0], spi_miso};
                    if (CPHA == 0) begin
                        // Shift out next bit
                        tx_buffer <= {tx_buffer[6:0], 1'b0};
                    end
                    bit_cnt <= bit_cnt + 1;
                    if (bit_cnt == 3'b111) begin
                        // All bits received, go back to idle
                        state <= STATE_IDLE;
                        ready <= 1'b1;
                    end else begin
                        // Continue transfer
                        state <= STATE_TRANSFER_WRITE;
                    end
                end
                default: begin
                    state <= STATE_IDLE;
                end
            endcase
        end
    end

    // SPI clock generation
    assign spi_clk = rst ? CPOL : (state == STATE_TRANSFER_READ ? ~CPOL : CPOL);
    // SPI MOSI output -- shift out the MSB first
    assign spi_mosi = tx_buffer[7];

    // External state signals
    assign busy = state[1]; // busy when in transfer state

endmodule