.PHONY: all clean

CBINDGEN_BIN:=cbindgen
CBINDGEN_HEADER=ios-src/sls-webgpu.h

all: $(CBINDGEN_HEADER)

$(CBINDGEN_HEADER):
	$(CBINDGEN_BIN) -o $@

clean:
	rm -f $(CBINDGEN_HEADER)
