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
	substreams run -e eth.substreams.pinax.network:443 ch_out -s 20423024 -t 20423025 -o json

.PHONY: gui
gui:
	substreams gui -e eth.substreams.pinax.network:443 ch_out -s 20441659 --production-mode

.PHONY: sql-setup
sql-setup:
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/default substreams.yaml

.PHONY: sql-run
sql-run:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/default substreams.yaml -e eth.substreams.pinax.network:443 20441659:20448859 --final-blocks-only --undo-buffer-size 10 --on-module-hash-mistmatch=warn --batch-block-flush-interval 10 --development-mode

.PHONY: deploy
deploy:
	graph build
	graph deploy --studio clock