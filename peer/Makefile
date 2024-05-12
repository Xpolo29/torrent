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
	cd src-tauri && $(CC) build



run: $(TARGET) $(TAURI_RSCS) 
	echo $^ > /dev/null
	npm run tauri dev
	cp ./src-tauri/target/debug/peerapp ./build/
	cp ./src-tauri/config.ini ./build

run-release: release $(TAURI_RSCS)
	echo $^ > /dev/null
	npm run tauri build
	cp ./src-tauri/target/release/peerapp ./build/
	cp ./src-tauri/config.ini ./build

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
	cd src-tauri && $(CC) build --release
