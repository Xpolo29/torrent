CC = cargo
TARGET = peer

SRCS = $(wildcard client/src/*.rs)

all: $(TARGET)


$(TARGET): $(SRCS)
	echo $^ > /dev/null
	cd client && $(CC) build
	cp client/target/debug/client ./$(TARGET)

run: $(TARGET)
	RUST_LOG=trace ./$(TARGET)

test: 
	echo no test for now

check: test
	./test

clean:
	rm -f $(TARGET)
	cd client && cargo clean

help:
	grep '^[a-zA-Z0-9_\-]*:' Makefile | sed 's/://'

