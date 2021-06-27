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
SPIRV_DIRS=$(dir $(SPIRV_OUT))
$(OUT_DIR)%.vert.spv: %.vert
	$(GLSLC)  $(GLSLCFLAGS) -o $@ $^
$(OUT_DIR)%.frag.spv: %.frag
	$(GLSLC)  $(GLSLCFLAGS) -o $@ $^

all: before $(SPIRV_OUT)

before:

clean:
	rm $(FRAG_OUT) $(VERT_OUT)