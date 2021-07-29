.PHONY=all clean before $(SPIRV_DIRS) copy_assets

ifeq ($(OS),Windows_NT)
.SHELL=powershell.exe
endif


GLSLC=glslangValidator
GLSLCFLAGS:=-g -V
MKDIR:=mkdir
ifneq ($(OS),Windows_NT)
FRAG_GLSL=src/shaders/main.frag
VERT_GLSL=src/shaders/main.vert
OUT_DIR:=./out
else
FRAG_GLSL=src\shaders\main.frag
VERT_GLSL=src\shaders\main.vert
OUT_DIR:=.\\target\\
endif


FRAG_OUT=$(FRAG_GLSL:%.frag=$(OUT_DIR)%.frag.spv)
VERT_OUT=$(VERT_GLSL:%.vert=$(OUT_DIR)%.vert.spv)
SPIRV_OUT=$(FRAG_OUT) $(VERT_OUT)
SPIRV_DIRS=$(dir $(SPIRV_OUT)) $(OUT_DIR)

$(OUT_DIR)%.vert.spv: %.vert
	$(GLSLC)  $(GLSLCFLAGS) -o $@ $^
$(OUT_DIR)%.frag.spv: %.frag
	$(GLSLC)  $(GLSLCFLAGS) -o $@ $^

all: before $(SPIRV_OUT) copy_assets
debug:
	@echo OUT_DIR $(OUT_DIR)
	@echo FRAG_GLSL $(FRAG_GLSL)
	@echo VERT_GLSL $(VERT_GLSL)
	@echo SPIRV_OUT $(SPIRV_OUT)
	@echo SPIRV_DIRS $(SPIRV_DIRS)

before:
ifneq ($(OS),Windows_NT)
	mkdir -p $(SPIRV_DIRS)
else
	@powershell -command "$(foreach dir,$(SPIRV_DIRS),mkdir -path $(dir) -force | out-null;)"
endif

copy_assets:
ifneq ($(OS),Windows_NT)
	cp -r public $(OUT_DIR)/public/
else
	@powershell -command "cp -Recurse -path .\\assets -Destination $(OUT_DIR)\\assets\\"
endif


clean:
	rm $(FRAG_OUT) $(VERT_OUT)