TARGET_DIR := /usr/local/bin

all: build

build:
	cargo build --release

clean:
	cargo clean

install: build
	sudo install -Dsm755 target/release/bibi -t $(TARGET_DIR)

uninstall:
	sudo rm "$(TARGET_DIR)/bibi"
