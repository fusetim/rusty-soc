# Makefile for building and simulating Verilog entities
# Target: Lattice ECP5 / ULX3S (ECP5-85k) board

# Tools
IVERILOG = iverilog
VVP = vvp
YOSYS = yosys
NEXTPNR = nextpnr-ecp5
ECPPACK = ecppack
VERILATOR = verilator

IVERILOG_FLAGS = -I/usr/local/share/silice/oss-cad-suite/share/yosys/ecp5 -g2012

# Files
DST = BUILD

# Board
BOARD_CONST = ../boards/ulx3s.lpf

# -- SpiMaster.v related build --
# Simulation iverilog
${DST}/SpiMaster.vvp: SpiMaster_tb.v SpiMaster.v | dst
	$(IVERILOG) -o ${DST}/SpiMaster.vvp -s SpiMaster_tb $^

${DST}/SpiMaster.vcd: ${DST}/SpiMaster.vvp | dst
	cd $(DST) && $(VVP) SpiMaster.vvp

# Post-synthesis simulation
${DST}/SpiMaster.json: SpiMaster.v | dst
	cd $(DST) && $(YOSYS) -p "read_verilog -sv ../SpiMaster.v" -p 'scratchpad -copy abc9.script.flow3 abc9.script; synth_ecp5 -abc9 -json SpiMaster.json -top SpiMaster'

${DST}/SpiMaster_synth.v: ${DST}/SpiMaster.json | dst
	$(YOSYS) -p 'read_json $^; write_verilog $@'

${DST}/SpiMaster_synth.vvp: SpiMaster_synth_tb.v ${DST}/SpiMaster_synth.v | dst 
	$(IVERILOG) $(IVERILOG_FLAGS) -o ${DST}/SpiMaster_synth.vvp -s SpiMaster_tb $^ `yosys-config --datdir/ecp5/cells_sim.v` 

${DST}/SpiMaster_synth.vcd: ${DST}/SpiMaster_synth.vvp | dst
	cd $(DST) && $(VVP) SpiMaster_synth.vvp
# -- End of SpiMaster.v related build --

# -- SpiMasterPeripheral.v related build --
# Simulation iverilog
${DST}/SpiMasterPeripheral.vvp: SpiMasterPeripheral_tb.v SpiMasterPeripheral.v SpiMaster.v | dst
	$(IVERILOG) -o ${DST}/SpiMasterPeripheral.vvp -s SpiMasterPeripheral_tb $^

${DST}/SpiMasterPeripheral.vcd: ${DST}/SpiMasterPeripheral.vvp | dst
	cd $(DST) && $(VVP) SpiMasterPeripheral.vvp

# Post-synthesis simulation
${DST}/SpiMasterPeripheral.json: SpiMasterPeripheral.v SpiMaster.v | dst
	cd $(DST) && $(YOSYS) -p "read_verilog -sv ../SpiMasterPeripheral.v ../SpiMaster.v" -p 'scratchpad -copy abc9.script.flow3 abc9.script; synth_ecp5 -abc9 -json SpiMasterPeripheral.json -top SpiMasterPeripheral'

${DST}/SpiMasterPeripheral_synth.v: ${DST}/SpiMasterPeripheral.json | dst
	$(YOSYS) -p 'read_json $^; write_verilog $@'

${DST}/SpiMasterPeripheral_synth.vvp: SpiMasterPeripheral_synth_tb.v ${DST}/SpiMasterPeripheral_synth.v | dst
	$(IVERILOG) $(IVERILOG_FLAGS) -o ${DST}/SpiMasterPeripheral_synth.vvp -s SpiMasterPeripheral_synth_tb $^ `yosys-config --datdir/ecp5/cells_sim.v` 

${DST}/SpiMasterPeripheral_synth.vcd: ${DST}/SpiMasterPeripheral_synth.vvp | dst
	cd $(DST) && $(VVP) SpiMasterPeripheral_synth.vvp

# Flash - build the bitstream for the SPI Flash peripheral testbench
${DST}/SpiMasterPeripheral_flash.json: SpiMasterPeripheral_flash_tb.v SpiMasterPeripheral.v SpiMaster.v | dst
	cd $(DST) && $(YOSYS) -p "read_verilog -sv ../SpiMasterPeripheral_flash_tb.v ../SpiMasterPeripheral.v ../SpiMaster.v" -p 'scratchpad -copy abc9.script.flow3 abc9.script; synth_ecp5 -abc9 -json SpiMasterPeripheral_flash.json -top top'

${DST}/SpiMasterPeripheral_flash.svf: ${DST}/SpiMasterPeripheral_flash.json | dst
	cd $(DST) && $(NEXTPNR) --85k --package CABGA381 --freq 25 --json SpiMasterPeripheral_flash.json --textcfg SpiMasterPeripheral_flash.config --lpf ../$(BOARD_CONST) --timing-allow-fail -r
	cd $(DST) && $(ECPPACK) --svf SpiMasterPeripheral_flash.svf SpiMasterPeripheral_flash.config SpiMasterPeripheral_flash.bit

${DST}/SpiMasterPeripheral_flash_synth.v: ${DST}/SpiMasterPeripheral_flash.json | dst
	$(YOSYS) -p 'read_json $^; write_verilog $@'

${DST}/SpiMasterPeripheral_flash_synth.vvp: ulx3s_tb.v ${DST}/SpiMasterPeripheral_flash_synth.v | dst
	$(IVERILOG) $(IVERILOG_FLAGS) -o ${DST}/SpiMasterPeripheral_flash_synth.vvp -s ulx3s_tb $^ `yosys-config --datdir/ecp5/cells_sim.v` 

${DST}/SpiMasterPeripheral_flash_synth.vcd: ${DST}/SpiMasterPeripheral_flash_synth.vvp | dst
	cd $(DST) && $(VVP) SpiMasterPeripheral_flash_synth.vvp

# -- End of SpiMasterPeripheral.v related build --

${DST}/build_synth.v: ${DST}/build.json | dst
	$(YOSYS) -p 'read_json $^; write_verilog $@'

${DST}/build.vvp: ulx3s_tb.v ${DST}/build.v | dst
	$(IVERILOG) $(IVERILOG_FLAGS) -o ${DST}/build.vvp -s ulx3s_tb $^ `yosys-config --datdir/ecp5/cells_sim.v`

${DST}/build.vcd: ${DST}/build.vvp | dst
	cd $(DST) && $(VVP) build.vvp

build-srun: ${DST}/build_synth.v
	

.PHONY: clean dst spimaster-sim spimaster-psim spimasterperi-sim spimasterperi-psim spimasterperi-flash

# Sim-link target
spimaster-sim: ${DST}/SpiMaster.vcd
spimaster-psim: ${DST}/SpiMaster_synth.vcd
spimasterperi-sim: ${DST}/SpiMasterPeripheral.vcd
spimasterperi-psim: ${DST}/SpiMasterPeripheral_synth.vcd
spimasterperi-flash: ${DST}/SpiMasterPeripheral_flash.svf

# Ensure destination directory exists
dst:
	mkdir -p $(DST)

# Clean
clean:
	rm -rf $(DST)