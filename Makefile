SRC=$(wildcard 0001-0050/*.rs)
LIBSRC=$(wildcard lib/*.rs)
LIBEULER=./lib/libeuler-*.so
TARGET=$(SRC:.rs=)
TESTS=./lib/euler.test

RUSTC_FLAGS=
LD_FLAGS=-L ./lib

all: $(TARGET)

$(LIBEULER): ./lib/euler.rc $(LIBSRC)
	rustc --lib $(RUSTC_FLAGS) $<

%: %.rs $(LIBEULER) $(LIBSRC)
	rustc $(RUSTC_FLAGS) $(LD_FLAGS) $< -o $@

%.test: %.rs $(LIBEULER)
	rustc --test $(RUSTC_FLAGS) $(LD_FLAGS) $< -o $@

./lib/euler.test: ./lib/euler.rc $(LIBSRC)
	rustc --test $(RUSTC_FLAGS) $(LD_FLAGS) $< -o $@

.PHONY: test clean

test: $(TESTS)
	@for exe in $(TESTS); do echo "$$exe"; ./$$exe; done

clean:
	$(RM) $(TARGET) $(LIBEULER) $(TESTS)

