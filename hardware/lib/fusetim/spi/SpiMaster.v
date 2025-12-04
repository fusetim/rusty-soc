module SPIMaster (
    input clk,
    input clkr, // CLK reference for SPI timing -- managed outside of this module

    // SPI GPIO interface
    output reg SPI_CLK,
    input  SPI_MISO,
    output SPI_MOSI,
    // SPI_CS is managed outside

    // SPI internal communication
    input [7:0] tx_buf_byte,
    output reg [7:0] rx_buf_byte,

    // SPI Signal indicators
    input transfer, // Enable until the transfer is complete (must be high until tx_complete is triggered)
    output reg tx_buf_busy, // High when the TX buffer is full (cannot accept new data)
    output reg rx_buf_ready, // High when a new byte has been received (only high for 1 clk cycle)
    output reg tx_complete, // High when a byte have been completely transfered (only high for 1 clk cycle)
);
    // In transmission bytes
    reg [7:0] tx_byte;
    reg [7:0] rx_byte;
    reg [2:0] bit_cnt;

    // Register the pos/neg edge of the SPI clk
    reg [1:0] SPI_CLKr = 2'b00;

    reg [3:0] state = 3'b111;

    parameter  S_IDLE     = 3'h0;
    parameter  S_WRITE1     = 3'h1;
    parameter  S_READ0    = 3'h2;
    parameter  S_READ1      = 3'h3;
    parameter  S_WRITE0      = 3'h4;

    always @(posedge clk) {
        SPI_CLKr <= {SPI_CLKr[0], clkr};
    }

    always @(posedge clk) {
        case(state)
            S_WRITE1:
            begin
                MOSI <= tx_byte[0];
                tx_byte <= tx_byte[7:1];
                state <= S_READ0;
            end
            S_READ0:
                // 0 states are waiting state for SPI clk 
                if (SPI_CLKr == 2'b01) begin // posedge
                    state <= S_READ1;
                end
            S_READ1:
            begin
                rx_byte <= {rx_byte[6:0], SPI_MISO};
                state <= S_WRITE0;
            end
            S_WRITE0:
            begin
                // 0 states are waiting state for SPI clk
                if (SPI_CLKr == 2'b10) begin // negedge
                    if (bit_cnt == 3'b000) begin
                        rx_buf_byte <= rx_byte;
                        rx_buf_ready <= 1'b1;
                        tx_buf_busy <= 1'b0;
                        state <= S_IDLE;             
                    end
                    else begin
                        bit_cnt <= bit_cnt - 1;
                        state <= S_WRITE1;
                    end
                end
            end
            S_IDLE:
            begin
                rx_buf_ready <= 1'b0;
                if (transfer) begin
                    tx_byte <= tx_buf_byte;
                    tx_buf_busy <= 1'b1;
                    bit_cnt <= 3'b000;
                    state <= S_WRITE1;
                end
            end
        endcase
    }

endmodule
