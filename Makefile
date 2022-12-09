build:
	cargo build --release

install:
	install -Dsm755 target/release/bibi -t /usr/bin

uninstall:
	rm "/usr/bin/bibi"
