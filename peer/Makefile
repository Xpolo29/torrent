CC = cargo
TARGET = peer

RS_SRCS = $(wildcard src-tauri/src/*.rs)

HTML_FILE = src/index.html
JS_FILE = src/main.js
CSS_FILE = src/styles.css

TAURI_RSCS = $(HTML_FILE) $(JS_FILE) $(CSS_FILE)

SRCS += $(RS_SRCS) $(TAURI_RSCS) 

all: $(TARGET)

$(TARGET): $(RS_SRCS)
	echo $^ > /dev/null
	cd src-tauri && $(CC) build -j 16

run-terminal: release $(TAURI_RSCS) 
	echo $^ > /dev/null
	npm run tauri build -j
	cp ./src-tauri/target/debug/peerapp ./build/
	cp ./src-tauri/config.ini ./build
	./build/peerapp -c ./build/config.ini -i terminal

run: $(TARGET) $(TAURI_RSCS) 
	echo $^ > /dev/null
	npm run tauri dev
	cp ./src-tauri/target/debug/peerapp ./build/
	cp ./src-tauri/config.ini ./build

run-release: release $(TAURI_RSCS)
	echo $^ > /dev/null
	npm run tauri build -j
	cp ./src-tauri/target/release/peerapp ./build/
	cp ./src-tauri/config.ini ./build
	./build/peerapp -c ./build/config.ini

test: 
	cd src-tauri
	RUST_TEST_THREADS=1 cargo test

check: test
	./test

clean:
	cd src-tauri && cargo clean
	rm build/*

help:
	grep '^[a-zA-Z0-9_\-]*:' Makefile | sed 's/://'

release: $(RS_SRCS) 
	echo $^ > /dev/null
	cd src-tauri && $(CC) build --release -j 16
