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
	substreams run -e eos.substreams.pinax.network:443 ch_out -s 386841287 -t 387014085 -o json

.PHONY: gui
gui:
	substreams gui -e eos.substreams.pinax.network:443 ch_out -s 386841287 -t 387014085

.PHONY: sql-setup
sql-setup:
	# EVM blocks
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/eos substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/wax substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/telos substreams.yaml

# Antelope blocks
.PHONY: sql-run-eos
sql-run-eos:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/eos substreams.yaml -e eos.substreams.pinax.network:443 386841287:387014085 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: deploy
deploy:
	graph build
	graph deploy --studio clock