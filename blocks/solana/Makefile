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
	substreams run -e mainnet.sol.streamingfast.io:443 ch_out -s 203096780 -t 203096781 -o json

.PHONY: gui
gui:
	substreams gui -e mainnet.sol.streamingfast.io:443 ch_out -s 203096780 -t 203096781

.PHONY: sql-setup
sql-setup:
	# EVM blocks
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/solana substreams.yaml

# Solanablocks
.PHONY: sql-run-solana
sql-run-solana:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/solana substreams.yaml -e mainnet.sol.streamingfast.io:443 203096780:203196780 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 1 --development-mode

.PHONY: sql-run-solana-blocks
sql-run-solana-blocks:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/solana substreams.yaml -e mainnet.sol.streamingfast.io:443 2: --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --params ch_out=blocks

.PHONY: deploy
deploy:
	graph build
	graph deploy --studio clock