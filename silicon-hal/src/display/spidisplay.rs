/// SSD1351 OLED display command IDs.
///
/// From the AdaFruit SSD1351 library / datasheet (https://learn.adafruit.com/adafruit-1-5-color-oled-breakout-board/downloads-and-links).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ssd1351CommandId {
    SetColumn = 0x15,
    SetRow = 0x75,
    WriteRam = 0x5C,
    ReadRam = 0x5D,
    SetRemap = 0xA0,
    StartLine = 0xA1,
    DisplayOffset = 0xA2,
    DisplayAllOff = 0xA4,
    DisplayAllOn = 0xA5,
    NormalDisplay = 0xA6,
    InvertDisplay = 0xA7,
    FunctionSelect = 0xAB,
    DisplayOff = 0xAE,
    DisplayOn = 0xAF,
    Precharge = 0xB1,
    DisplayEnhance = 0xB2,
    ClockDiv = 0xB3,
    SetVsl = 0xB4,
    SetGpio = 0xB5,
    Precharge2 = 0xB6,
    SetGray = 0xB8,
    UseLut = 0xB9,
    PrechargeLevel = 0xBB,
    Vcomh = 0xBE,
    ContrastAbc = 0xC1,
    ContrastMaster = 0xC7,
    MuxRatio = 0xCA,
    CommandLock = 0xFD,
    HorizScroll = 0x96,
    StopScroll = 0x9E,
    StartScroll = 0x9F,
}

/// Command trait for sending commands to the SSD1351 display.
pub trait Ssd1351Command<const ARG: usize> {
    fn command_id(&self) -> Ssd1351CommandId;
    fn command_data(&self) -> [u8; ARG];
}

pub mod cmd {
    use super::{Ssd1351Command, Ssd1351CommandId};
    use bitfield::bitfield;

    /// Command to turn the display on.
    pub struct DisplayOnCommand;
    impl Ssd1351Command<0> for DisplayOnCommand {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::DisplayOn
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 0] {
            []
        }
    }

    /// Command to turn the display off.
    pub struct DisplayOffCommand;

    impl Ssd1351Command<0> for DisplayOffCommand {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::DisplayOff
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 0] {
            []
        }
    }

    /// Command to (un)lock the command interface.
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CommandLockCommand {
        /// Unlock the command interface
        UnlockIC = 0x12,
        /// Lock the command interface
        ///
        /// This prevents any commands from being accepted until unlocked again (this is the only command accepted).
        LockIC = 0x16,
        /// Lock the restricted command set
        ///
        /// This prevents certain commands (configuration mostly) from being accepted until unlocked again.
        LockRestricted = 0xB0,
        /// Unlock the restricted command set
        ///
        /// This unlocks the restricted command set (A2,B1,B3,BB,BE,C1) for use.
        UnlockRestricted = 0xB1,
    }

    impl Ssd1351Command<1> for CommandLockCommand {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::CommandLock
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 1] {
            [*self as u8]
        }
    }

    /// Command to configure the internal Clock Divider and Oscillator frequency.
    pub struct ClockDivCommand {
        /// The clock divider value (only the lower 4 bits are used)
        divider: u8,
        /// The oscillator frequency (only the lower 4 bits are used)
        osc_freq: u8,
    }

    impl ClockDivCommand {
        /// Creates a new ClockDivCommand
        ///
        /// # Arguments
        ///
        /// * `divider` - The clock divider value (only the lower 4 bits are used)
        /// * `osc_freq` - The oscillator frequency (only the lower 4 bits are used)
        #[inline(always)]
        pub const fn new(divider: u8, osc_freq: u8) -> Self {
            Self {
                divider: divider & 0x0F,
                osc_freq: osc_freq & 0x0F,
            }
        }
    }

    impl Ssd1351Command<1> for ClockDivCommand {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::ClockDiv
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 1] {
            [self.osc_freq << 4 | self.divider]
        }
    }

    /// Command to set the multiplex ratio.
    pub struct MuxRatioCommand {
        /// The multiplex ratio (valid values are 15 to 127)
        ratio: u8,
    }

    impl MuxRatioCommand {
        /// Creates a new MuxRatioCommand
        ///
        /// # Arguments
        ///
        /// * `ratio` - The multiplex ratio (valid values are 15 to 127)
        ///
        /// # Returns
        ///
        /// * `Ok(MuxRatioCommand)` if the ratio is valid
        /// * `Err(())` if the ratio is invalid
        #[inline(always)]
        pub const fn new(ratio: u8) -> Result<Self, ()> {
            if ratio < 15 || ratio > 127 {
                Err(())
            } else {
                Ok(Self { ratio })
            }
        }
    }

    impl Ssd1351Command<1> for MuxRatioCommand {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::MuxRatio
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 1] {
            [self.ratio]
        }
    }

    /// Command to set the display offset.
    pub struct DisplayOffsetCommand {
        /// The display offset value
        offset: u8,
    }

    impl DisplayOffsetCommand {
        /// Creates a new DisplayOffsetCommand
        ///
        /// # Arguments
        ///
        /// * `offset` - The display offset value (0-127)
        ///
        /// # Returns
        ///
        /// * `Ok(DisplayOffsetCommand)` if the offset is valid
        /// * `Err(())` if the offset is invalid
        #[inline(always)]
        pub const fn new(offset: u8) -> Result<Self, ()> {
            if offset > 127 {
                Err(())
            } else {
                Ok(Self { offset })
            }
        }
    }

    impl Ssd1351Command<1> for DisplayOffsetCommand {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::DisplayOffset
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 1] {
            [self.offset]
        }
    }

    /// GPIO configuration options for the SetGpioCommand.
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum DisplayGpioConfig {
        InputDisabled = 0x00,
        InputEnabled = 0x01,
        OutputLow = 0x02,
        OutputHigh = 0x03,
    }

    /// Command to set the GPIO configuration.
    pub struct SetGpioCommand {
        gpio0: DisplayGpioConfig,
        gpio1: DisplayGpioConfig,
    }

    impl SetGpioCommand {
        /// Creates a new SetGpioCommand
        ///
        /// # Arguments
        ///
        /// * `gpio0` - The configuration for GPIO0
        /// * `gpio1` - The configuration for GPIO1
        #[inline(always)]
        pub const fn new(gpio0: DisplayGpioConfig, gpio1: DisplayGpioConfig) -> Self {
            Self { gpio0, gpio1 }
        }
    }

    impl Ssd1351Command<1> for SetGpioCommand {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::SetGpio
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 1] {
            [(self.gpio1 as u8) << 2 | (self.gpio0 as u8)]
        }
    }

    /// Parallel interface options for the FunctionSelectCommand.
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum FunctionSelectParallelOption {
        Parallel8Bit = 0x00,
        Parallel16Bit = 0x01,
        Parallel18Bit = 0x03,
    }

    /// Command to set the function selection.
    pub struct FunctionSelectCommand {
        parallel_option: FunctionSelectParallelOption,
        external_vdd: bool,
    }

    impl FunctionSelectCommand {
        /// Creates a new FunctionSelectCommand
        ///
        /// # Arguments
        ///
        /// * `parallel_option` - The parallel interface option
        /// * `external_vdd` - Whether to use external VDD
        #[inline(always)]
        pub const fn new(
            parallel_option: FunctionSelectParallelOption,
            external_vdd: bool,
        ) -> Self {
            Self {
                parallel_option,
                external_vdd,
            }
        }
    }

    impl Ssd1351Command<1> for FunctionSelectCommand {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::FunctionSelect
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 1] {
            [
                // Bit 7:6 - Parallel option
                // Bit 0 - External VDD
                (self.parallel_option as u8) << 6 | (self.external_vdd as u8),
            ]
        }
    }

    /// Command to set the pre-charge period (for phase 1 and phase 2).
    pub struct PrechargeCommand {
        /// The phase 1 period (0-15)
        phase1: u8,
        /// The phase 2 period (0-15)
        phase2: u8,
    }
    impl PrechargeCommand {
        /// Creates a new PrechargeCommand
        ///
        /// # Arguments
        ///
        /// * `phase1` - The phase 1 period (0-15)
        /// * `phase2` - The phase 2 period (0-15)
        #[inline(always)]
        pub const fn new(phase1: u8, phase2: u8) -> Self {
            Self {
                phase1: phase1 & 0x0F,
                phase2: phase2 & 0x0F,
            }
        }
    }

    impl Ssd1351Command<1> for PrechargeCommand {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::Precharge
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 1] {
            [self.phase2 << 4 | self.phase1]
        }
    }

    /// Command to set the VCOMH voltage.
    pub struct VcomhCommand {
        /// The VCOMH voltage level (0-7)
        level: u8,
    }

    impl VcomhCommand {
        /// Creates a new VcomhCommand
        ///
        /// # Arguments
        ///
        /// * `level` - The VCOMH voltage level (0-7)
        #[inline(always)]
        pub const fn new(level: u8) -> Self {
            Self {
                level: level & 0x07,
            }
        }
    }

    impl Ssd1351Command<1> for VcomhCommand {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::Vcomh
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 1] {
            [self.level]
        }
    }

    /// Command to set the contrast for the RGB channels.
    pub struct ContrastAbcCommand {
        /// Contrast for A channel
        pub channel_a: u8,
        /// Contrast for B channel
        pub channel_b: u8,
        /// Contrast for C channel
        pub channel_c: u8,
    }

    impl ContrastAbcCommand {
        /// Creates a new ContrastAbcCommand
        ///
        /// # Arguments
        ///
        /// * `channel_a` - Contrast for A channel
        /// * `channel_b` - Contrast for B channel
        /// * `channel_c` - Contrast for C channel
        #[inline(always)]
        pub const fn new(channel_a: u8, channel_b: u8, channel_c: u8) -> Self {
            Self {
                channel_a,
                channel_b,
                channel_c,
            }
        }
    }

    impl Ssd1351Command<3> for ContrastAbcCommand {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::ContrastAbc
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 3] {
            [self.channel_a, self.channel_b, self.channel_c]
        }
    }

    /// Command Normal Display
    pub struct NormalDisplayCommand;

    impl Ssd1351Command<0> for NormalDisplayCommand {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::NormalDisplay
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 0] {
            []
        }
    }

    /// Command Invert Display
    pub struct InvertDisplayCommand;

    impl Ssd1351Command<0> for InvertDisplayCommand {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::InvertDisplay
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 0] {
            []
        }
    }

    /// Command to set contrast master
    pub struct ContrastMasterCommand {
        /// The contrast master ratio (k/16, k = 0..=16)
        level: u8,
    }

    impl ContrastMasterCommand {
        /// Creates a new ContrastMasterCommand
        ///
        /// # Arguments
        ///
        /// * `level` - The contrast master ratio (k/16, k = 0..=16)
        #[inline(always)]
        pub const fn new(level: u8) -> Self {
            Self {
                level: level & 0x0F,
            }
        }
    }

    impl Ssd1351Command<1> for ContrastMasterCommand {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::ContrastMaster
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 1] {
            [self.level]
        }
    }

    /// Command to set the Segment Low Voltage (VSL)
    pub struct SetVslCommand;

    impl Ssd1351Command<3> for SetVslCommand {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::SetVsl
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 3] {
            // Value are hardcoded as per Adafruit example
            [0xA0, 0xB5, 0x55]
        }
    }

    /// Command to set the Second Pre-charge period
    pub struct Precharge2Command {
        /// The second pre-charge period (0-15)
        period: u8,
    }

    impl Precharge2Command {
        /// Creates a new Precharge2Command
        ///
        /// # Arguments
        ///
        /// * `period` - The second pre-charge period (0-15)
        #[inline(always)]
        pub const fn new(period: u8) -> Self {
            Self {
                period: period & 0x0F,
            }
        }
    }

    impl Ssd1351Command<1> for Precharge2Command {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::Precharge2
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 1] {
            [self.period]
        }
    }

    bitfield! {
        /// Command to set the Remap and Color Depth
        pub struct SetRemapCommand(u8);
        /// Set the Horizontal / Vertical increment (0 = Horizontal, 1 = Vertical)
        pub inc_dir, set_inc_dir: 0;
        /// Set the column address mapping
        pub col_addr_map, set_col_addr_map: 1;
        /// Set the color sequence
        pub color_seq, set_color_seq: 2;
        _,_: 3; // Reserved
        /// Set the scan direction
        pub scan_dir, set_scan_dir: 4;
        /// Enable or disable COM split odd even
        pub com_split_odd_even, set_com_split_odd_even: 5;
        /// Set the color depth
        /// 00/01 = 16-bit (RGB565)
        /// 10 = 18-bit (RGB666)
        /// 11 = 18-bit (16-bit format 2) (RGB666)
        pub color_depth, set_color_depth: 7,6;
    }

    impl Default for SetRemapCommand {
        #[inline(always)]
        fn default() -> Self {
            let mut cmd = SetRemapCommand(0);
            cmd.set_inc_dir(false); // Horizontal increment
            cmd.set_col_addr_map(false); // Column address 0 is mapped to SEG0
            cmd.set_color_seq(false); // Color sequence: A->B->C
            cmd.set_scan_dir(false); // Scan from COM[0] to COM[N-1]
            cmd.set_com_split_odd_even(true); // Enable COM split odd even
            cmd.set_color_depth(0b00); // 16-bit color depth (RGB565)
            cmd
        }
    }

    impl Ssd1351Command<1> for SetRemapCommand {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::SetRemap
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 1] {
            [self.0]
        }
    }

    /// Command to set the Start Line
    pub struct StartLineCommand {
        /// The start line (0-127)
        line: u8,
    }

    impl StartLineCommand {
        /// Creates a new StartLineCommand
        ///
        /// # Arguments
        ///
        /// * `line` - The start line (0-127)
        #[inline(always)]
        pub const fn new(line: u8) -> Self {
            Self { line: line & 0x7F }
        }
    }

    impl Ssd1351Command<1> for StartLineCommand {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::StartLine
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 1] {
            [self.line]
        }
    }

    /// Command to set the Column Address
    pub struct SetColumnCommand {
        /// The start column (0-127)
        start: u8,
        /// The end column (0-127)
        end: u8,
    }

    impl SetColumnCommand {
        /// Creates a new SetColumnCommand
        ///
        /// # Arguments
        ///
        /// * `start` - The start column (0-127)
        /// * `end` - The end column (0-127)
        ///
        /// # Returns
        ///
        /// * `Ok(SetColumnCommand)` if the start and end columns are valid
        /// * `Err(())` if the start or end columns are invalid
        #[inline(always)]
        pub const fn new(start: u8, end: u8) -> Result<Self, ()> {
            if start > 127 || end > 127 || start > end {
                Err(())
            } else {
                Ok(Self { start, end })
            }
        }
    }

    impl Ssd1351Command<2> for SetColumnCommand {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::SetColumn
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 2] {
            [self.start, self.end]
        }
    }

    /// Command to set the Row Address
    pub struct SetRowCommand {
        /// The start row (0-127)
        start: u8,
        /// The end row (0-127)
        end: u8,
    }

    impl SetRowCommand {
        /// Creates a new SetRowCommand
        ///
        /// # Arguments
        ///
        /// * `start` - The start row (0-127)
        /// * `end` - The end row (0-127)
        ///
        /// # Returns
        ///
        /// * `Ok(SetRowCommand)` if the start and end rows are valid
        /// * `Err(())` if the start or end rows are invalid
        #[inline(always)]
        pub const fn new(start: u8, end: u8) -> Result<Self, ()> {
            if start > 127 || end > 127 || start > end {
                Err(())
            } else {
                Ok(Self { start, end })
            }
        }
    }

    impl Ssd1351Command<2> for SetRowCommand {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::SetRow
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 2] {
            [self.start, self.end]
        }
    }

    /// Command to setup for writting to RAM
    pub struct WriteRamCommand;
    impl Ssd1351Command<0> for WriteRamCommand {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::WriteRam
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 0] {
            []
        }
    }

    /// Command to setup for reading from RAM
    pub struct ReadRamCommand;
    impl Ssd1351Command<0> for ReadRamCommand {
        #[inline(always)]
        fn command_id(&self) -> Ssd1351CommandId {
            Ssd1351CommandId::ReadRam
        }
        #[inline(always)]
        fn command_data(&self) -> [u8; 0] {
            []
        }
    }
}
