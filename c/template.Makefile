.PHONY: clean

main: main.o
	cc main.o -o main

main.o: clean
	cc -c main.c -o main.o

clean:
	rm -rf *.o main

run: clean main
	./main
