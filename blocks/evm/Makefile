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
	substreams-sink-files run eth.substreams.pinax.network:443 substreams.yaml map_events s3://pinax/eth/d091b44a01fb21c3a882e56a49659cda9382cb48?region=us-east-1 21200000: --encoder parquet --development-mode --parquet-default-column-compression snappy --file-block-count 1

.PHONY: hash
hash:
	substreams info --json | jq '.modules[0].hash'