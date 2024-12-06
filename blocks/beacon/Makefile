NETWORK ?= eth-cl
HASH = $(shell substreams info --json | jq -r '.modules[0].hash')

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
	substreams-sink-files run "$(NETWORK).substreams.pinax.network:443" substreams.yaml map_events './out' 8626176:8626676 --encoder parquet --file-block-count 100 --development-mode

.PHONY: s3
s3:
	substreams-sink-files run "$(NETWORK).substreams.pinax.network:443" substreams.yaml map_events "s3://pinax/$(NETWORK)/$(HASH)?region=us-east-1" 0: --encoder parquet --parquet-default-column-compression snappy

