GLSLC=glslc
GLSLCFLAGS:=-g
MKDIR:=mkdir
.PHONY=all clean before $(SPIRV_DIRS)
FRAG_GLSL=src/shaders/main.frag
VERT_GLSL=src/shaders/main.vert
OUT_DIR:=


FRAG_OUT=$(FRAG_GLSL:%.frag=$(OUT_DIR)%.frag.spv)
VERT_OUT=$(VERT_GLSL:%.vert=$(OUT_DIR)%.vert.spv)
SPIRV_OUT=$(FRAG_OUT) $(VERT_OUT)
ifneq ($(OS),Windows_NT)
SPIRV_DIRS=$(dir $(SPIRV_OUT))
else
SPIRV_DIRS=$(patsubst /,\\,$(dir $(SPIRV_OUT)))
endif

$(OUT_DIR)%.vert.spv: %.vert
	$(GLSLC)  $(GLSLCFLAGS) -o $@ $^
$(OUT_DIR)%.frag.spv: %.frag
	$(GLSLC)  $(GLSLCFLAGS) -o $@ $^

all: before $(SPIRV_OUT)

before: $(SPIRV_DIRS)
ifneq ($(OS),Windows_NT)
	mkdir -p $(SPIRV_DIRS)
else
	@powershell -command "$(foreach dir,$(SPIRV_DIRS),mkdir -path $(dir) -force | out-null;)"
endif

clean:
	rm $(FRAG_OUT) $(VERT_OUT)