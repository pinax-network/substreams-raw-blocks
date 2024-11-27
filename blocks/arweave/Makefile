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
	substreams-sink-files run arweave.substreams.pinax.network:443 substreams.yaml map_events './out' 1500000:1500500 --encoder parquet --file-block-count 100 --development-mode
