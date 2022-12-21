all:
	@echo "!!! Note that this was not tested as I only compiled using Bazel !!!"
	${CC} -Iinc -Llib -lcomplex_polynomials src/main.c -o c_caller

run:
	./c_caller

clean:
	rm c_caller

.PHONY: all run clean
