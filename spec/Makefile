TYPST       := typst
TYPST_FLAGS := --font-path assets/fonts --features html --root .

.PHONY: all

all: dist/index.pdf

dist/%.pdf: src/%.typ
	$(TYPST) compile $< $@ $(TYPST_FLAGS)