RUSTC=rustc

all: average echo greete

average: average.rs
	$(RUSTC) average.rs -o average

echo: echo.rs
	$(RUSTC) echo.rs -o echo

greete: greete.rs
	$(RUSTC) greete.rs -o greete

clean:
	rm average echo greete
