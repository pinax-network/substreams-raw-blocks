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

.PHONY: sql-setup
sql-setup:
	# EVM beacon blocks
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/beacon substreams.yaml 

.PHONY: sql-run-eth-cl
sql-run-eth-cl:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/beacon substreams.yaml -e eth-cl.substreams.pinax.network:443 8626176:8666176 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode
