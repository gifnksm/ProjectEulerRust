SRC=$(wildcard 0001-0050/*.rs)
LIBEULER=./lib/libeuler-5465da761d20cd31-0.0.so
TARGET=$(SRC:.rs=)

RUSTC_FLAGS=--warn-unused-imports
LD_FLAGS=-L ./lib

all: $(TARGET)

$(LIBEULER): ./lib/euler.rc
	rustc $(RUSTC_FLAGS) $<

%: %.rs $(LIBEULER)
	rustc $(RUSTC_FLAGS) $(LD_FLAGS) $<

.PHONY: clean
clean:
	$(RM) $(TARGET) $(LIBEULER)

