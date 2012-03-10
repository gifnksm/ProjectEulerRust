SRC=$(wildcard 0001-0050/*.rs)
LIBSRC=$(wildcard lib/*.rs)
LIBEULER=./lib/libeuler-aa21bafe3914d2be-0.0.so
TARGET=$(SRC:.rs=)
TESTS=$(SRC:.rs=.test) $(LIBSRC:.rs=.test)

RUSTC_FLAGS=--warn-unused-imports
LD_FLAGS=-L ./lib

all: $(TARGET)

$(LIBEULER): ./lib/euler.rc $(LIBSRC)
	rustc $(RUSTC_FLAGS) $<

%: %.rs $(LIBEULER)
	rustc $(RUSTC_FLAGS) $(LD_FLAGS) $<

%.test: %.rs $(LIBEULER)
	rustc --test $(RUSTC_FLAGS) $(LD_FLAGS) $< -o $@

.PHONY: test
test: $(TESTS)

.PHONY: clean
clean:
	$(RM) $(TARGET) $(LIBEULER) $(TESTS)

