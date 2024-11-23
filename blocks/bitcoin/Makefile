.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams info
	substreams pack
	substreams graph

.PHONY: protogen
protogen:
	substreams protogen

.PHONY: parquet
parquet:
	substreams-sink-files run bitcoin.substreams.pinax.network:443 substreams.yaml map_events './out' 800000:800010 --encoder parquet --file-block-count 1 --development-mode --parquet-default-column-compression snappy
