CC = cargo
TARGET = peer

SRCS = $(wildcard src-tauri/src/*.rs)

# all: $(TARGET)

#
# $(TARGET): $(SRCS)
# 	echo $^ > /dev/null
# 	$(CC) build
# 	cp target/debug/client ./$(TARGET)
#
run: 
	# RUST_LOG=trace ./$(TARGET)
	npm run tauri dev

test: 
	cd src-tauri
	RUST_TEST_THREADS=1 cargo test

check: test
	./test

clean:
	cd src-tauri
	cargo clean

help:
	grep '^[a-zA-Z0-9_\-]*:' Makefile | sed 's/://'

release: 
	npm run tauri build
	mv ./src-tauri/target/release/peerapp ./build/
	cp ./src-tauri/config.ini ./build
