SRC=$(wildcard 0001-0050/*.rs)
LIBSRC=$(wildcard lib/*.rs)
LIBEULER=./lib/libeuler.so
TARGET=$(SRC:.rs=)

RUSTC_FLAGS=--warn-unused-imports
LD_FLAGS=-L ./lib

all: $(TARGET)

$(LIBEULER): ./lib/euler.rc $(LIBSRC)
	rustc $(RUSTC_FLAGS) $< -o $@

%: %.rs $(LIBEULER)
	rustc $(RUSTC_FLAGS) $(LD_FLAGS) $<

.PHONY: clean
clean:
	$(RM) $(TARGET) $(LIBEULER) lib/libeuler-*.so

