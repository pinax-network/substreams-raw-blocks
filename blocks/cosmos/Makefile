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
	substreams gui substreams.yaml -e mainnet.injective.streamingfast.io:443 map_events -s 80000000

.PHONY: protogen
protogen:
	substreams protogen

.PHONY: parquet
parquet:
	substreams-sink-files run mainnet.injective.streamingfast.io:443 substreams.yaml map_events './out' 80000000:80005000 --encoder parquet --file-block-count 100 --development-mode
