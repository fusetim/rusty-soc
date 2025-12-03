// Memory mapped peripheral addresses
pub const PERIPHERAL_BASE: usize = 0x10000;     // 10000000000000000
pub const PERIPHERAL_ADDR_MASK: usize = 0x0FF0; // 01111110000000000
pub const PERIPHERAL_REG_MASK: usize  = 0x000F; // 00000001111111100

// GPIO Peripheral
pub const GPIO_BASE: usize     = PERIPHERAL_BASE | (0b000001 << 10);
pub const GPIO_REG_LEDS: usize = GPIO_BASE | (0x00 << 2);
pub const GPIO_REG_BTNS: usize = GPIO_BASE | (0x01 << 2);

// DAC Peripheral (AUDIO)
pub const DAC_BASE: usize        = PERIPHERAL_BASE | (0b000010 << 10);

// OLED Peripheral (TODO)
pub const OLED_BASE: usize         = PERIPHERAL_BASE | (0b000011 << 10);

// UART Peripheral (TODO)
pub const UART_BASE: usize        = PERIPHERAL_BASE | (0b000100 << 10);

// SPI Peripheral (TODO)
pub const SPI_BASE: usize        = PERIPHERAL_BASE | (0b000101 << 10);

