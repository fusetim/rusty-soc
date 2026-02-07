# Silicon

Silicon is the main SoC firmware crate for the Rusty SoC project. It is the actual application
that runs on the Rusty SoC hardware. It implements a fully-featured audio player that streams PCM audio
from an SDCard to a PWM-based Audio DAC (with hardware streaming support), and make use of the OLED display
and the buttons for a graphical interface.

This project is part of a larger course taught at Telecom Nancy (France) about designing RISC-V based
systems using different tools like VHDL, Silice, Verilog and FPGA boards such as the ULX3S (Lattice ECP5 board)
and the DE1-SoC (Altera Cyclone V board). *See [requirements](https://gist.github.com/sylefeb/ea83119db9a07d45db2cae49fc3ea471) for more details.*

## Features

***!**: Mark a feature as planned*

#### Assignment - Minimal set of features to be implemented
- [x] **!** Menu with Album list (e.g. one per directory on the SDCard)
- [x] **!** Play/Pause control using buttons
- [x] **!** On end of track, automatically gets back to the Album list
- [x] **!** Display track art when playing
- [x] **!** LED effect (baked in the hardware) -- *it might not be related to the music*  
      *Known issue (https://github.com/fusetim/rusty-soc/issues/3): Energy meter works only when volume is at max, otherwise the DC offset is not 128 and the energy calculation is incorrect. This could be fixed by adding a volume control that adjusts the DC offset accordingly.*

#### Assignment Bonus - Additional features to implement if time permits
- [x] **!** Seek control using buttons (e.g. skip forward/backward by 10s)  
      *Known issue (https://github.com/fusetim/rusty-soc/issues/1): Seeking may not work correctly near the start or end of the file*
- [ ] **!** On-launch sound effect and/or sound made on the hardware sound generator
- [x] **!** Color screen
- [x] **!** Animation while playing (OLED display and/or LEDs)
- [ ] Sound in the menu *(not clear if it's a sound effect on button press, or if music should keep playing in the menu)*
- [x] **!** Volume control using buttons and LED indicator
- [ ] White (Brown? Pink?) noise generator 
- [ ] Any easter egg you can think of!

#### Additional Features
- [x] Rust-based firmware (basically everything that was provided in C/C++ has found a Rust equivalent or been reimplemented)
- [x] Support for "static/global" memory (ro_data/data and bss sections)
- [x] 2x Hardware SPI Masters (one for the SDCard, one for the OLED display) based on an home-made SPI peripheral (in Verilog)
- [x] PLL-based clock generation :
  - *25MHz for the core,*
  - *80MHz for the SPI0 peripheral (40MHz for the SDCard),*
  - *20MHz for the SPI1 peripheral (10MHz for the OLED display)*
- [x] 48kHz PCM Audio (8bit, Mono) instead of 8kHz
- [x] Framebuffer-less graphics using the `embedded-graphics` crate and home-made SSD1351 driver (SPI-based)
- [ ] Stereo Audio support
- [ ] Timer0 peripheral (1MHz clock) for timekeeping

## Building and Running

See the main [Rusty SoC README](../README.md) for instructions on how to build and run the firmware.