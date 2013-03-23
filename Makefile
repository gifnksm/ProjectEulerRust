UNAME=$(shell uname)

ifeq ($(OS), Windows_NT)
LIBEXT=.dll
EXEEXT=.exe
else ifeq ($(UNAME), Linux)
LIBEXT=.so
EXEEXT=
else ifeq ($(UNAME), Darwin)
LIBEXT=.dylib
EXEEXT=
else
$(error Unknown OS $(OS) or UNAME $(UNAME))
endif

SRC=$(wildcard *-*/*.rs)
LIBSRC=$(wildcard lib/*.rs)
LIBEULER=./lib/libeuler-*$(LIBEXT)
TARGET=$(SRC:.rs=$(EXEEXT))
TESTS=./lib/euler.test$(EXEEXT)

RUSTC_FLAGS=
LD_FLAGS=-L ./lib
TEST_RUSTC_FLAGS=

all: $(TARGET)

$(LIBEULER): ./lib/euler.rc $(LIBSRC)
	rustc --lib $(RUSTC_FLAGS) $<

%: %.rs $(LIBEULER) $(LIBSRC)
	rustc $(RUSTC_FLAGS) $(LD_FLAGS) $< -o $@

./lib/%.test: ./lib/%.rc $(LIBSRC)
	rustc --test $(RUSTC_FLAGS)  $(TEST_RUSTC_FLAGS) $(LD_FLAGS) $< -o $@

.PHONY: test clean

test: $(TESTS)
	@for exe in $(TESTS); do echo "$$exe"; ./$$exe; done

clean:
	$(RM) $(TARGET) $(LIBEULER) $(TESTS)

