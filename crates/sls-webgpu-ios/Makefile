.PHONY: all clean

CBINDGEN_OUT=sls_webgpu_ios/sls_webgpu_rs.h

CRATENAME=sls-webgpu-ios
RS_SRC=$(glob src/**/*.rs ) src/*.rs

all: $(CBINDGEN_OUT)

$(CBINDGEN_OUT): | $(RS_SRC) cbindgen.toml
	cbindgen --config cbindgen.toml --crate $(CRATENAME) --output $@

clean:
	rm $(CBINDGEN_OUT)