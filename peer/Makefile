CC = cargo
TARGET = peer

all: $(TARGET)

$(TARGET):
	cd client && $(CC) build
	cp client/target/debug/client ./$(TARGET)

run: $(TARGET)
	./$(TARGET)

test: 
	echo no test for now

check: test
	./test

clean:
	rm -f $(TARGET)
	cd client && cargo clean

help:
	grep '^[a-zA-Z0-9_\-]*:' Makefile | sed 's/://'

