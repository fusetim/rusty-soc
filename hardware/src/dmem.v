`define DEPTH 28672
`define ADDR_SIZE 32
`define DATA_SIZE 32
`define WORD_SIZE 4

/// Data Memory (RAM)
///
/// DEPTH: Memory size in words
/// ADDR_SIZE: Size in bits of the address
/// DATA_SIZE: Size in bits of a memory cell (ie a word)
/// WORD_SIZE: Size in byte of a word
module dmem 
(
    input       [31:0] addr,
    output reg  [31:0] rdata,
    input       [3:0]  wenable,
    input       [31:0] wdata,
    input              clock
);
    (* no_rw_check *) 
    reg  [`DATA_SIZE-1:0] buffer[`DEPTH-1:0];

    // Read logic
    always @(posedge clock) begin
        rdata <= buffer[addr];
    end

    // Write logic
    integer i;
    always @(posedge clock) begin
        for (i = 0; i < `WORD_SIZE; i = i + 1) begin
            if (wenable[i]) begin
                buffer[addr][i*8+:8] <= wdata[i*8+:8];
            end
        end
    end

    // Memory cell initialization
    initial begin 
        $readmemh("../../target/riscv32i-unknown-none-elf/release/silicon.mem", buffer);
    end
endmodule