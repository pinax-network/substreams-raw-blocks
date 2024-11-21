.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack
	substreams graph
	substreams info

.PHONY: protogen
protogen:
	substreams protogen

.PHONY: parquet
parquet:
	substreams-sink-files run eth-cl.substreams.pinax.network:443 substreams.yaml map_events './out' 8626176:8626676 --encoder parquet --file-block-count 100 --development-mode
