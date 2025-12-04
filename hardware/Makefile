BOARD = ulx3s
ARGS ?=
OUTPUT_DIR ?= BUILD

.phony: soc clean

soc: src/soc.si
ifeq ($(BOARD),verilator)
	silice-make.sh -s src/soc.si -b $(BOARD) -p basic,oled -o $(OUTPUT_DIR) $(ARGS)
else
	silice-make.sh -s src/soc.si -b $(BOARD) -p basic,audio,oled,buttons,sdcard -o $(OUTPUT_DIR) $(ARGS)
endif

clean:
	make -C firmware clean
	rm -rf BUILD_*
	rm -f *.lpp
