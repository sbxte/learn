.PHONY: clean


main: main.o
	cc -O3 -lraylib main.o -o main

main.o: clean
	cc -O3 -c main.c -o main.o

clean:
	rm -rf *.o main

run: clean main
	./main
