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
	substreams run -e eth.substreams.pinax.network:443 ch_out -s 20437424 -t 20423025 -o json

.PHONY: gui
gui:
	substreams gui -e eth.substreams.pinax.network:443 ch_out -s 1

.PHONY: sql-setup
sql-setup:

	substreams-sink-sql setup clickhouse://default:default@localhost:9000/eth substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/eth substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/bsc substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/base substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/polygon substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/arbone substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/avalanche substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/mode substreams.yaml

.PHONY: sql-run-eth
sql-run-eth:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/eth substreams.yaml -e eth.substreams.pinax.network:443 20444295:20451460 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: sql-run-base
sql-run-base:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/base substreams.yaml -e base.substreams.pinax.network:443 17926927:17970127 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: sql-run-bsc
sql-run-bsc:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/bsc substreams.yaml -e bsc.substreams.pinax.network:443 41025119:41053841 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: sql-run-polygon
sql-run-polygon:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/polygon substreams.yaml -e polygon.substreams.pinax.network:443 60124533:60163432 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: sql-run-arbone
sql-run-arbone:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/arbone substreams.yaml -e arbone.substreams.pinax.network:443 238821160: --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: sql-run-avalanche
sql-run-avalanche:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/avalanche substreams.yaml -e avalanche.substreams.pinax.network:443 48771562:48813123 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: sql-run-mode
sql-run-mode:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/mode substreams.yaml -e mode.substreams.pinax.network:443 11212266: --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: deploy
deploy:
	graph build
	graph deploy --studio clock