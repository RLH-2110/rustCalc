
PRIMARY_SOURCE := main.rs
SOURCES := $(wildcard *.rs)
OUTPUT := calc.elf

all: $(OUTPUT)

$(OUTPUT): $(SOURCES)
	rustc $(PRIMARY_SOURCE) -o $(OUTPUT)

run: all
	./$(OUTPUT) $(wordlist 2, $(words $(MAKECMDGOALS)), $(MAKECMDGOALS))

clean:
	rm -f $(OUTPUT)

.PHONY: all clean run
