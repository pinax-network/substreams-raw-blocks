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
	substreams run -e eos.substreams.pinax.network:443 graph_out -s 392479516 -o json

.PHONY: gui
gui:
	substreams gui -e eos.substreams.pinax.network:443 graph_out -s 392479516 --production-mode

.PHONY: sql-setup
sql-setup:
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/eos substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/wax substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/telos substreams.yaml

.PHONY: sql-run-eos
sql-run-eos:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/eos substreams.yaml -e eos.substreams.pinax.network:443 386841287:387014085 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 1 --development-mode

.PHONY: sql-run-eos-test
sql-run-eos-test:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/eos substreams.yaml -e eos.substreams.pinax.network:443 386946800:386946810 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 1 --development-mode

.PHONY: sql-run-eos-blocks
sql-run-eos-blocks:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/eos substreams.yaml -e eos.substreams.pinax.network:443 2: --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --params ch_out=blocks

.PHONY: sql-run-wax
sql-run-wax:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/wax substreams.yaml -e wax.substreams.pinax.network:443 322339177:322511977 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: deploy
deploy:
	graph build
	graph deploy --studio clock