SOURCES := main.rs
OUTPUT := calc.elf

all: $(OUTPUT)

$(OUTPUT): $(SOURCES)
	rustc $(SOURCES) -o $(OUTPUT)

run: all
	./$(OUTPUT) $(wordlist 2, $(words $(MAKECMDGOALS)), $(MAKECMDGOALS))

clean:
	rm -f $(OUTPUT)

.PHONY: all clean run
