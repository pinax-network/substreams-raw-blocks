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
	substreams run substreams.yaml -e mainnet.injective.streamingfast.io:443 ch_out -s 80000000 -o json

.PHONY: gui
gui:
	substreams gui substreams.yaml -e mainnet.injective.streamingfast.io:443 ch_out -s 80000000

.PHONY: sql-setup
sql-setup:
	# EVM blocks
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/cosmos substreams.yaml

.PHONY: sql-run-cosmos
sql-run-cosmos:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/cosmos substreams.yaml -e mainnet.injective.streamingfast.io:443 80000000:81000000 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn  --batch-block-flush-interval 1 --development-mode


.PHONY: deploy
deploy:
	graph build
	graph deploy --studio clock