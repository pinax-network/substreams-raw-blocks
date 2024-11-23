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
	substreams gui -e eth.substreams.pinax.network:443 map_events -s 20500000 -t 20500001

.PHONY: protogen
protogen:
	substreams protogen

.PHONY: parquet
parquet:
	substreams-sink-files run eth.substreams.pinax.network:443 substreams.yaml map_events ./out 20500000:20500001 --encoder parquet --file-block-count 1 --development-mode --parquet-default-column-compression snappy

.PHONY: s3
s3:
	substreams-sink-files run eth.substreams.pinax.network:443 substreams.yaml map_events s3://pinax/eth/7605ec58a864d4016ed2245b066d8a8c09b891d3?region=us-east-1 21252718: --encoder parquet --file-block-count 1 --development-mode --parquet-default-column-compression snappy

.PHONY: hash
hash:
	substreams info --json | jq '.modules[0].hash'