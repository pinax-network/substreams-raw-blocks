.PHONY: all
all:
	make build
	make pack

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: pack
pack:
	substreams pack

.PHONY: gui
gui:
	substreams gui -e wax.substreams.pinax.network:443 graph_out -s -1
