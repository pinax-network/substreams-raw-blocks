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
	substreams-sink-files run mainnet.btc.streamingfast.io:443 substreams.yaml map_events './out' 0:500 --encoder parquet --file-block-count 100 --development-mode
