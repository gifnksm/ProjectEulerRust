SRC=$(wildcard 0001-0050/*.rs)
TARGET=$(SRC:.rs=)
RUSTC_FLAGS=--warn-unused-imports

%: %.rs
	rustc $(RUSTC_FLAGS) $<

all: $(TARGET)

.PHONY: clean
clean:
	$(RM) $(TARGET)

