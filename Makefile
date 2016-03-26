VERSION=0.1.2
ZIP=uenc-${VERSION}.zip
USER=kumabook
API_KEY=xxxx
URL=https://api.bintray.com/content/kumabook/homebrew/uenc/${VERSION}/uenc.zip

default: package

package:
	cargo build --release
	zip -j ${ZIP} target/release/udec target/release/uenc

publish:
	curl -T ${ZIP} -u${USER}:${API_KEY} ${URL}

clean:
	cargo clean
	rm *.zip

.PHONY: clean pakcage publish default
