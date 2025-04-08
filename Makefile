
PRIMARY_SOURCE := main.rs
SOURCES := $(wildcard *.rs)
OUTPUT := calc.elf

all: $(OUTPUT)

$(OUTPUT): $(SOURCES)
	rustc $(PRIMARY_SOURCE) -o $(OUTPUT)

run: all
	./$(OUTPUT) $(wordlist 2, $(words $(MAKECMDGOALS)), $(MAKECMDGOALS))

release: $(SOURCES)
	rustc $(PRIMARY_SOURCE) -O -o $(OUTPUT)

test: release
	rustc test.rs -o test.elf
	./test.elf

clean:
	rm -f $(OUTPUT)
	rm -f *.pdb
	rm -f test.elf

.PHONY: all clean run release test
