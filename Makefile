VERSION=0.1.2

default: package

package:
	cargo build --release
	zip -j uenc-${VERSION}.zip target/release/udec target/release/uenc

clean:
	cargo clean
	rm *.zip

.PHONY: clean pakcage default
