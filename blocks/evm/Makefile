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
	substreams run -e eth.substreams.pinax.network:443 ch_out -s 6912 -t 13775 -o jsonl

.PHONY: gui
gui:
	substreams gui -e eth.substreams.pinax.network:443 ch_out -s 20000000 -t 20000010

.PHONY: sql-setup
sql-setup:
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/default substreams.yaml

.PHONY: sql-run
sql-run:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/default substreams.yaml -e eth.substreams.pinax.network:443 1:10000 --final-blocks-only --undo-buffer-size 100 --on-module-hash-mistmatch=warn --batch-block-flush-interval 10

.PHONY: deploy
deploy:
	graph build
	graph deploy --studio clock