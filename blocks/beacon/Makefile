.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: pack
pack:
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths google

.PHONY: parquet
parquet:
	substreams-sink-files run eth-cl.substreams.pinax.network:443 substreams.yaml map_events './out' 8626176:8626676 --encoder parquet --file-block-count 100 --development-mode
