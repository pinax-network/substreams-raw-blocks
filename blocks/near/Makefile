.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack
	substreams info
	substreams graph

.PHONY: protogen
protogen:
	substreams protogen

.PHONY: parquet
parquet:
	substreams-sink-files run near.substreams.pinax.network:443 substreams.yaml map_events './out' 100000000:100000500 --encoder parquet --file-block-count 100 --development-mode
