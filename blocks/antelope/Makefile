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
	substreams gui . -e eos.substreams.pinax.network:443 map_events -s 407431959 -t -1 --network eos --production-mode

.PHONY: protogen
protogen:
	substreams protogen

.PHONY: parquet
parquet:
	rm -rf out state.yaml && substreams-sink-files run eos.substreams.pinax.network:443 substreams.yaml map_events './out' 386841287:386841288 --encoder parquet --file-block-count 1 --development-mode

.PHONY: s3
s3:
	substreams-sink-files run eos.substreams.pinax.network:443 substreams.yaml map_events s3://pinax/eos/84951f7ebf3e55a9e1c3a75f095f6a8ba95175b1?region=us-east-1 407431959: --encoder parquet --parquet-default-column-compression snappy
