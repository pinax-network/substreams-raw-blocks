NETWORK ?= eos
HASH = $(shell substreams info --json | jq -r '.modules[0].hash')

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
	substreams gui . -e "$(NETWORK).substreams.pinax.network:443" map_events -s 407300000 -t 0 --network $(NETWORK) --production-mode

.PHONY: protogen
protogen:
	substreams protogen

.PHONY: parquet
parquet:
	substreams-sink-files run "$(NETWORK).substreams.pinax.network:443" substreams.yaml map_events "./out" 414957000: --encoder parquet --file-block-count 1 --parquet-default-column-compression snappy

.PHONY: s3
s3:
	substreams-sink-files run "$(NETWORK).substreams.pinax.network:443" substreams.yaml map_events "s3://pinax/$(NETWORK)/$(HASH)?region=us-east-1" 0: --encoder parquet --parquet-default-column-compression snappy --file-block-count 1000
