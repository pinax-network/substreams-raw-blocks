NETWORK ?= base
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
	substreams gui . -e "$(NETWORK).substreams.pinax.network:443" map_events -s 24450126 --network $(NETWORK)

.PHONY: protogen
protogen:
	substreams protogen

.PHONY: parquet
parquet:
	rm -rf state.yaml && substreams-sink-files run "$(NETWORK).substreams.pinax.network:443" substreams.yaml map_events "./out" 21200000:21200001 --encoder parquet --file-block-count 1 --development-mode --parquet-default-column-compression snappy

.PHONY: s3
s3:
	substreams-sink-files run "$(NETWORK).substreams.pinax.network:443" substreams.yaml map_events "s3://pinax/$(NETWORK)/$(HASH)?region=us-east-1" 0: --encoder parquet --parquet-default-column-compression snappy --file-block-count 100 --state-store "$(NETWORK).state.yaml"
