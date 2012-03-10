SRC=$(wildcard 0001-0050/*.rs)
LIBSRC=$(wildcard lib/*.rs)
LIBEULER=./lib/libeuler-*.so
TARGET=$(SRC:.rs=)
TESTS=$(SRC:.rs=.test) $(LIBSRC:.rs=.test)

RUSTC_FLAGS=
LD_FLAGS=-L ./lib

all: $(TARGET)

$(LIBEULER): ./lib/euler.rc $(LIBSRC)
	rustc --lib $(RUSTC_FLAGS) $<

%: %.rs $(LIBEULER)
	rustc $(RUSTC_FLAGS) $(LD_FLAGS) $< -o $@

%.test: %.rs $(LIBEULER)
	rustc --test $(RUSTC_FLAGS) $(LD_FLAGS) $< -o $@

.PHONY: test
test: $(TESTS)

.PHONY: clean
clean:
	$(RM) $(TARGET) $(LIBEULER) $(TESTS)

