SRC=$(wildcard 0001-0050/*.rs)
LIBSRC=$(wildcard lib/*.rs)
LIBEULER=./lib/libeuler-aa21bafe3914d2be-0.0.so
TARGET=$(SRC:.rs=)

RUSTC_FLAGS=--warn-unused-imports
LD_FLAGS=-L ./lib

all: $(TARGET)

$(LIBEULER): ./lib/euler.rc $(LIBSRC)
	rustc $(RUSTC_FLAGS) $<

%: %.rs $(LIBEULER)
	rustc $(RUSTC_FLAGS) $(LD_FLAGS) $<

.PHONY: clean
clean:
	$(RM) $(TARGET) $(LIBEULER)

