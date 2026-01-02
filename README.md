# Rusty SoC

Rusty SoC is a from-scratch System on Chip (SoC) design in Silice+Verilog.
It is designed to be flashable onto a FPGA board like the ULX3S, and it provides a "complete" SoC 
experience for Rust developers (it includes a Peripheral Access Crate (PAC), a Hardware Abstraction Layer (HAL),
and an example applications).

## Features

**Hardware Components**:
- **RISC-V RV32I Core**: A simple and efficient 32-bit RISC-V core implemented in Silice ([The Ice-V](hardware/lib/silice/projects/ice-v/IceV.md)).
  It currently supports the RV32I instruction set, and comes with no interrupts nor branch prediction.
- **Common Peripherals**: 
  - [x] 2x home-made SPI Masters (SPI0 connected to an SDCard, SPI1 connected to the OLED display)
  - [x] 2x Audio "8-bit" DAC (PWM-based)
  - [x] 1x Hardware Audio Streamer (48kHz - 8bit - PCM - Mono)
  - [x] 8x Output Pins (Onboard LEDs)
  - [x] 6x Input Pins (Onboard Buttons)
  - [ ] 1x Timer (Timer0 - 1MHz clock)
- **CMSYS-SVD description**: The SoC is fully described using the CMSYS-SVD format, allowing automatic generation of the Peripheral Access Crate (PAC) using `svd2rust` (see [`hardware/svd.xml`](hardware/svd.xml)).

**Software Components**:
- **Bare-Metal Rust Support**: Write applications in Rust without an operating system.
- **Peripheral Access Crate (PAC)**: Auto-generated PAC for the Rusty SoC peripherals using `svd2rust` (see the [`silicon-pac` crate](./silicon-pac/)).
- **Hardware Abstraction Layer (HAL)**: A simple HAL to interact with the SoC peripherals (see the [`silicon-hal` crate](./silicon-hal/)).
- **embedded-hal Compatibility**: Leverage the `embedded-hal` traits for peripheral access (such as GPIO and SPI).
- **embedded-graphics Support**: Use the `embedded-graphics` crate to draw on the OLED display.
- **Example Applications**: 
  - An audio player that streams PCM audio from an SDCard to the Audio DAC with a graphical interface (see the [`silicon` crate](./silicon/)).

## Getting Started

### Prerequisites

This project is based on the Open-Source project [Silice](https://github.com/sylefeb/Silice) for the hardware part.
Make sure you have the following tools installed:
- Silice
- Yosys
- Nextpnr
- Rust Nightly toolchain with `cargo` and `rustup` (see [rustup.rs](https://rustup.rs/))
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