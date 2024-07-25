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

.PHONY: deploy
deploy:
	graph build
	graph deploy --studio clock