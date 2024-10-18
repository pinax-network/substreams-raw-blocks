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
	substreams run -e solana.substreams.pinax.network:443 ch_out -s 203100000 -o json

.PHONY: cache
cache:
	substreams gui ./raw-blocks-solana-v0.1.0.spkg -e solana.substreams.pinax.network:443 ch_out -s 203100000 -t 203200000 --production-mode

.PHONY: cache-foundational
cache-foundational:
	substreams gui https://github.com/streamingfast/substreams-foundational-modules/releases/download/solana-common-v0.3.0/solana-common-v0.3.0.spkg -e solana.substreams.pinax.network:443 blocks_without_votes -s 203100000 -t 203200000 --production-mode

.PHONY: gui
gui:
	substreams gui ./substreams.yaml -e solana.substreams.pinax.network:443 ch_out -s 203100000

.PHONY: sql-setup
sql-setup:
	# EVM blocks
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/solana substreams.yaml

.PHONY: sql-run-solana
sql-run-solana:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/solana ./raw-blocks-solana-v0.1.0.spkg -e solana.substreams.pinax.network:443 203100000:203200000 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 1

.PHONY: sql-run-solana-blocks
sql-run-solana-blocks:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/solana substreams.yaml -e solana.substreams.pinax.network:443 2: --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --params ch_out=blocks

.PHONY: deploy
deploy:
	graph build
	graph deploy --studio clock