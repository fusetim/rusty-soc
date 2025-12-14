#!/usr/bin/env bash
set -x
echo "Running post-build script..."

# Get the path to the build artifact
BUILD_ARTIFACT="$1"
OUTPUT_DIR="$(dirname "$BUILD_ARTIFACT")"
OUTPUT_FILE="$(basename "$BUILD_ARTIFACT")"

CMD_AS="riscv64-unknown-elf-as"
CMD_LD="riscv64-unknown-elf-ld"
CMD_GCC="riscv64-unknown-elf-gcc"
CMD_OBJCOPY="riscv64-unknown-elf-objcopy"
CMD_RANLIB="riscv64-unknown-elf-ranlib"

# Compile the entrypoint (asm)
echo "Assembling entrypoint..."
${CMD_AS} -march=rv32i -mabi=ilp32 -o "$OUTPUT_DIR/entry.o" silicon/entry.s

# If OUTPUT_FILE = "silicon", then we can assemble all the things for flashing on FPGA
if [[ "$OUTPUT_FILE" == "silicon" ]]; then
    echo "Preparing FPGA bitstream and related files..."
    # Set bash to fail on error
    set -e

    #$CMD_LD --gc-sections -m elf32lriscv -b elf32-littleriscv -Tsilicon/memory.x --no-relax -o "$OUTPUT_DIR/silicon.elf" "$OUTPUT_DIR/entry.o" "$OUTPUT_DIR/libsilicon.a"
    #$CMD_RANLIB "$OUTPUT_DIR/libsilicon.a"
    $CMD_OBJCOPY -O verilog "$BUILD_ARTIFACT" "$OUTPUT_DIR/silicon.hex"
    $CMD_OBJCOPY -O binary "$BUILD_ARTIFACT" "$OUTPUT_DIR/silicon.bin"
    echo "Generated silicon.hex and silicon.bin for FPGA."

    # Print the full path to the silicon.hex file
    echo "FPGA Hex File: $PWD/$OUTPUT_DIR/silicon.hex"
fi

echo "Post-build script completed."