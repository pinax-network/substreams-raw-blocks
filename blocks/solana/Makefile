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

.PHONY: run
run:
	substreams run substreams.yaml -e solana.substreams.pinax.network:443 map_events -s 203100000 -o json  --network solana

.PHONY: protogen
protogen:
	substreams protogen

.PHONY: cache
cache:
	substreams gui substreams.yaml -e solana.substreams.pinax.network:443 map_events -s 203100000 -t 203200000 --production-mode  --network solana

.PHONY: cache-foundational
cache-foundational:
	substreams gui https://github.com/streamingfast/substreams-foundational-modules/releases/download/solana-common-v0.3.0/solana-common-v0.3.0.spkg -e solana.substreams.pinax.network:443 blocks_without_votes -s 203100000 -t 203200000 --production-mode

.PHONY: gui
gui:
	substreams gui substreams.yaml -e solana.substreams.pinax.network:443 map_events -s 311081162 --network solana

.PHONY: parquet
parquet:
	substreams-sink-files run solana.substreams.pinax.network:443 substreams.yaml map_events './out' 311081162: --encoder parquet --file-block-count 50 --development-mode

.PHONY: deploy
deploy:
	graph build
	graph deploy --studio clock