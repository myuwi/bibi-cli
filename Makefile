TARGET_DIR := /usr/local/bin

build:
	cargo build --release

install: build
	sudo install -Dsm755 target/release/bibi -t $(TARGET_DIR)

uninstall:
	sudo rm "$(TARGET_DIR)/bibi"
