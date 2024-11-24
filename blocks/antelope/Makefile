.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack
	substreams info
	substreams graph

.PHONY: gui
gui:
	substreams gui . -e eos.substreams.pinax.network:443 map_events -s 386946801 -t 386946802 --network eos

.PHONY: protogen
protogen:
	substreams protogen

.PHONY: parquet
parquet:
	substreams-sink-files run eos.substreams.pinax.network:443 substreams.yaml map_events './out' 386841287:386841787 --encoder parquet --file-block-count 100 --development-mode
