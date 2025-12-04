# Rusty SoC

Rusty SoC is a from-scratch implementation of a System on Chip (SoC) architecture 
based on open-source components. It is designed to be flashable onto a FPGA board
like the ULX3S, and it provides a "complete" SoC experience for Rust developers.

## Features

**Hardware Components**:
- **RISC-V RV32I Core**: A simple and efficient 32-bit RISC-V core implemented in Silice ([The Ice-V](hardware/lib/silice/projects/ice-v/IceV.md)).
- **Common Peripherals**: *UART*, SPI, "GPIO" for basic input/output operations.
- **Memory-Mapped I/O**: Framebuffer (128x128 RGB565), Audio output (PWM-based), and more.

**Software Components**:
- **Bare-Metal Rust Support**: Write applications in Rust without an operating system.
- **embedded-hal Compatibility**: Leverage the `embedded-hal` traits for peripheral access.
- [ ] **Bootloader**: A simple bootloader to initialize the system and load applications from SDCard.

## Getting Started

### Prerequisites

This project is based on the Open-Source project [Silice](https://github.com/sylefeb/Silice) for the hardware part.
Make sure you have the following tools installed:
- Silice
- Yosys
- Nextpnr
- Rust toolchain with `cargo` and `rustup` (see [rustup.rs](https://rustup.rs/))
- RISC-V RV32i target for Rust: `rustup target add riscv32i-unknown-none-elf`
- riscv64-unknown-elf-gcc toolchain for linking steps
- make

### Building the Firmware

The first step is to build the Rust firmware. You can do this by running:

```bash
cargo run -p silicon --release --target riscv32i-unknown-none-elf
```

This will compile the Rust code and produce a hex file that can be loaded onto the FPGA (silicon.hex).

### Building the Hardware

Next, you need to build the hardware design using Silice. Navigate to the `hardware` directory and run:

```bash
cd hardware
make soc BOARD=ulx3s
```

If you have the ULX3S board connected, it should have flashed the design automatically. If not, you can manually flash it using:

```bash
openFPGALoader -b ulx3s BUILD/build.bit
```

### Enjoy!

If everything went well, you should see the Rusty SoC booting up on your FPGA board.