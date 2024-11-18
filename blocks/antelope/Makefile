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
	substreams run -e eos.substreams.pinax.network:443 ch_out -s 386946801 -t 386946802 -o json

.PHONY: gui
gui:
	substreams gui -e eos.substreams.pinax.network:443 ch_out -s 386946801 -t 386946802

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths google,sf

.PHONY: parquet-eos
parquet-eos:
	substreams-sink-files run eos.substreams.pinax.network:443  substreams.yaml map_events './out' 386841287:386841787 --encoder parquet --file-block-count 100 --development-mode

.PHONY: parquet-wax
parquet-wax:
	substreams-sink-files run wax.substreams.pinax.network:443 substreams.yaml map_events './out' 32233917:32234417 --encoder parquet --file-block-count 100 --development-mode

# Antelope blocks
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