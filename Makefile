all: libchan.so main chan

clean:
	rm -f chan main libchan.so

chan: chan.rs
	rustc -g chan.rs

libchan.so: chan.rs
	rustc -g chan.rs --crate-type dylib

main: main.c libchan.so chan.h
	gcc -Wall main.c -L. -lchan -o main

.PHONY: all clean
