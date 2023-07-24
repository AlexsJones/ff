.PHONY: local-install

local-install:
	cargo install --path .
release:
	jreleaser assemble -grs
	jreleaser config