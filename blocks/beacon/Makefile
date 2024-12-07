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

.PHONY: gui
gui:
	substreams gui . -e "$(NETWORK).substreams.pinax.network:443" map_events -s 9000000 -t 0 --network $(NETWORK)

.PHONY: parquet
parquet:
	substreams-sink-files run "$(NETWORK).substreams.pinax.network:443" substreams.yaml map_events './out' 9000000: --encoder parquet --parquet-default-column-compression snappy --file-block-count 100 --state-store state-parquet.yaml

.PHONY: s3
s3:
	substreams-sink-files run "$(NETWORK).substreams.pinax.network:443" substreams.yaml map_events "s3://pinax/$(NETWORK)/$(HASH)?region=us-east-1" 0: --encoder parquet --parquet-default-column-compression snappy --file-block-count 100

