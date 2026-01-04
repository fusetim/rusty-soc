`timescale 1ns / 1ps
`ifndef SIMULATION
/// A simple PLL module to generate the multiple clocks
/// needed for the SPI clocks.
///
/// This module takes in a 25MHz reference clock and generates
/// 80MHz, 40MHz, 20MHz, 5MHz clocks using a PLL.
module spi_pll(
  input clk_25, 
  input rst_n,
  output wire clk_80,
  output wire clk_40,
  output wire clk_20,
  output wire clk_5
);

  // Instantiate the PLL primitive.
  (* FREQUENCY_PIN_CLKI="25" *)
  (* FREQUENCY_PIN_CLKOP="80" *)
  (* FREQUENCY_PIN_CLKOS="40" *)
  (* FREQUENCY_PIN_CLKOS2="20" *)
  (* FREQUENCY_PIN_CLKOS3="5" *)
  (* ICP_CURRENT="12" *) (* LPF_RESISTOR="8" *) (* MFG_ENABLE_FILTEROPAMP="1" *) (* MFG_GMCREF_SEL="2" *)
  EHXPLLL#(
    ////////
    /// General wiring
    ////////

    // Use the primary output as the feedback signal.
    .FEEDBK_PATH("CLKOP"),
    // Allow external PLL resets, so we can hook up rst_n.
    .PLLRST_ENA("ENABLED"),
    // For this example we don't need standby mode or dynamic
    // phase control.
    .STDBY_ENABLE("DISABLED"),
    .DPHASE_SOURCE("DISABLED"),
    // We're going to use all four PLL outputs, and we don't
    // care about toggling them individually, so statically
    // enable all of them.
    .CLKOP_ENABLE("ENABLED"),
    .CLKOS_ENABLE("ENABLED"),
    .CLKOS2_ENABLE("ENABLED"),
    .CLKOS3_ENABLE("ENABLED"),
    // We don't care about the PLL lock signal for this
    // example, so just set it to non-sticky mode arbitrarily.
    .PLL_LOCK_MODE(0),
    .INT_LOCK_STICKY("DISABLED"),

    ////////
    /// Frequency configuration
    ////////

    // Configure all four outputs. I used ecppll to compute the
    // appropriate divider values:
    //
    //  ecppll -i 25 --clkout0 160 --clkout1 80 --clkout2 20 --clkout3 5
    //
    .CLKI_DIV(5),      // REF CLK DIV: 25MHz -> 5MHz
    .CLKFB_DIV(16),    // FB DIV: 5MHz -> 80MHz
    .CLKOP_DIV(7),     // 80MHz out1
    .CLKOS_DIV(14),    // 40MHz out2
    .CLKOS2_DIV(28),   // 20MHz out3
    .CLKOS3_DIV(112),  // 5MHz out4

    ////////
    /// Phase configuration
    ////////

    // You can't really see phase offsets in megahertz signals with
    // the naked eye, so this demo sets a 0° offset across the board.
    .CLKOP_CPHASE(6),
    .CLKOP_FPHASE(0),
    .CLKOP_TRIM_DELAY(0),
    .CLKOP_TRIM_POL("RISING"),

    .CLKOS_CPHASE(13),
    .CLKOS_FPHASE(0),
    .CLKOS_TRIM_DELAY(0),
    .CLKOS_TRIM_POL("RISING"),

    .CLKOS2_CPHASE(27),
    .CLKOS2_FPHASE(0),

    .CLKOS3_CPHASE(111),
    .CLKOS3_FPHASE(0),
  ) pll(
    ////////
    /// I/O pin wiring
    ////////

    // The PLL module's reset is active high, so we need to invert our
    // external active-low reset.
    .RST(~rst_n),
    // The PLL's reference is the external 25MHz clock.
    .CLKI(clk_25),
    // We configured CLKOP as the feedback output. That's our 80MHz
    // clock.
    .CLKFB(clk_80),

    // The output clocks signals.
    .CLKOP(clk_80),
    .CLKOS(clk_40),
    .CLKOS2(clk_20),
    .CLKOS3(clk_5),

    // Tie unused inputs to 0.
    .ENCLKOP(0),
    .ENCLKOS(0),
    .ENCLKOS2(0),
    .ENCLKOS3(0),
    .STDBY(0),
    .PHASESEL1(0),
    .PHASESEL0(0),
    .PHASEDIR(0),
    .PHASESTEP(0),
    .PHASELOADREG(0),
    .PLLWAKESYNC(0),

    // Unused outputs, left floating.
    .LOCK()
  );
endmodule
`else
module spi_pll(
  input clk_25, 
  input rst_n,
  output wire clk_80,
  output wire clk_40,
  output wire clk_20,
  output wire clk_5
);

  // In simulation, just pass through the 25MHz clock.
  assign clk_80 = clk_25;
  assign clk_40 = clk_25;
  assign clk_20 = clk_25;
  assign clk_5 = clk_25;

endmodule
`endif