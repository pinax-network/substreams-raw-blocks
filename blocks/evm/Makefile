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
	substreams gui . -e eth.substreams.pinax.network:443 map_events -s 21200000 -t 21200001 --network mainnet

.PHONY: protogen
protogen:
	substreams protogen

.PHONY: parquet
parquet:
	rm -rf out state.yaml && substreams-sink-files run eth.substreams.pinax.network:443 substreams.yaml map_events ./out 21200000:21200001 --encoder parquet --file-block-count 1 --development-mode --parquet-default-column-compression snappy

.PHONY: s3
s3:
	substreams-sink-files run eth.substreams.pinax.network:443 substreams.yaml map_events s3://pinax/eth/a7483e216fa6fc6753e7d6a9809620036fcd077d?region=us-east-1 21200000: --encoder parquet --development-mode --parquet-default-column-compression snappy --file-block-count 1

.PHONY: hash
hash:
	substreams info --json | jq '.modules[0].hash'