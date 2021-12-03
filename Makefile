build:
	cargo build --release

clean:
	rm -rf target/ Cargo.lock

install:
	cp target/release/anime /usr/local/bin/

uninstall:
	rm /usr/local/bin/anime