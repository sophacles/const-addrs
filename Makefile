.PHONY: doc
doc: export RUSTDOCFLAGS= --cfg docsrs
doc:
	echo ":${RUSTDOCFLAGS}:"
	cargo +nightly doc  --all-features

.PHONY: serv
serv:
	darkhttpd target/doc/ --port 8080

.PHONY: test
test:
	cargo test --all-features
	cargo +nightly test --all-features
