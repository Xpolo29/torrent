CC = cargo
TARGET = peer

SRCS = $(wildcard src/*.rs)

all: $(TARGET)


$(TARGET): $(SRCS)
	echo $^ > /dev/null
	$(CC) build
	cp target/debug/client ./$(TARGET)

run: $(TARGET)
	RUST_LOG=trace ./$(TARGET)

test: 
	RUST_TEST_THREADS=1 cargo test

check: test
	./test

clean:
	rm -f $(TARGET)
	cargo clean

help:
	grep '^[a-zA-Z0-9_\-]*:' Makefile | sed 's/://'

release: $(SRCS)
	echo $^ > /dev/null
	$(CC) build --release
	cp target/release/client ./$(TARGET)
