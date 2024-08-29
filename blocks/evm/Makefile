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
	substreams run -e eth.substreams.pinax.network:443 ch_out -s 2060000 -t 2060001 -o json

.PHONY: gui
gui:
	substreams gui -e eth.substreams.pinax.network:443 ch_out -s 20500000 -t 20501001 --production-mode

.PHONY: sql-setup
sql-setup:
	# EVM blocks
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/eth substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/ethtest substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/eth substreams.yaml --cursors-table cursors1
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/eth substreams.yaml --cursors-table cursors2
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/eth substreams.yaml --cursors-table cursors3
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/base substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/bsc substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/polygon substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/arbone substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/mode substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/zora substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/xai substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/linea substreams.yaml

	# testnets
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/arbsepolia substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/sepolia substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/chapel substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/holesky substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/amoy substreams.yaml

	# not detailed EVM blocks
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/avalanche substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/optimism substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/blast substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/boba substreams.yaml
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/fantom substreams.yaml

	# not compatible with EVM blocks
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/injective substreams.yaml

# EVM blocks
.PHONY: sql-run-eth
sql-run-eth:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/eth substreams.yaml -e eth.substreams.pinax.network:443 20444295:20451460 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: sql-run-eth-test
sql-run-eth-test:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/ethtest substreams.yaml -e eth.substreams.pinax.network:443 20500000:20501001 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: sql-run-eth1
sql-run-eth1:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/eth substreams.yaml -e eth.substreams.pinax.network:443 1:14164145 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --cursors-table cursors1 --params ch_out=blocks

.PHONY: sql-run-eth2
sql-run-eth2:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/eth substreams.yaml -e eth.substreams.pinax.network:443 14164145:17865817 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --cursors-table cursors2 --params ch_out=blocks

.PHONY: sql-run-eth3
sql-run-eth3:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/eth substreams.yaml -e eth.substreams.pinax.network:443 17865817:20543112 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --cursors-table cursors3 --params ch_out=blocks

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
	substreams-sink-sql run clickhouse://default:default@localhost:9000/arbone substreams.yaml -e arbone.substreams.pinax.network:443 238821160:239165328 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: sql-run-mode
sql-run-mode:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/mode substreams.yaml -e mode.substreams.pinax.network:443 11237809:11281009 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: sql-run-zora
sql-run-zora:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/zora substreams.yaml -e zora.substreams.pinax.network:443 17974681:18017880 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: sql-run-xai
sql-run-xai:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/xai substreams.yaml -e xai.substreams.pinax.network:443 23596657:23895839 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: sql-run-linea
sql-run-linea:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/linea substreams.yaml -e linea.substreams.pinax.network:443 7619882:7663070 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

# EVM Testnets
.PHONY: sql-run-arbsepolia
sql-run-arbsepolia:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/arbsepolia substreams.yaml -e arbsepolia.substreams.pinax.network:443 68562960:68762761 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: sql-run-chapel
sql-run-chapel:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/chapel substreams.yaml -e chapel.substreams.pinax.network:443 42641054:42669854 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: sql-run-sepolia
sql-run-sepolia:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/sepolia substreams.yaml -e sepolia.substreams.pinax.network:443 6425676:6432059 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: sql-run-holesky
sql-run-holesky:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/holesky substreams.yaml -e holesky.substreams.pinax.network:443 2055608:2062315 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: sql-run-amoy
sql-run-amoy:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/amoy substreams.yaml -e amoy.substreams.pinax.network:443 10256772:10294903 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

# Not detailed EVM blocks
.PHONY: sql-run-avalanche
sql-run-avalanche:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/avalanche substreams.yaml -e avalanche.substreams.pinax.network:443 48771562:48813123 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: sql-run-optimism
sql-run-optimism:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/optimism substreams.yaml -e optimism.substreams.pinax.network:443 123511924: --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: sql-run-blast
sql-run-blast:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/blast substreams.yaml -e blast.substreams.pinax.network:443 6916693:6959893 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: sql-run-boba
sql-run-boba:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/boba substreams.yaml -e boba.substreams.pinax.network:443 5819180:5862380 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

# Not working with EVM blocks
.PHONY: sql-run-injective
sql-run-injective:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/injective substreams.yaml -e mainnet.injective.streamingfast.io:443 80303917: --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

# Not ready yet
.PHONY: sql-run-fantom
sql-run-fantom:
	substreams-sink-sql run clickhouse://default:default@localhost:9000/fantom substreams.yaml -e fantom.substreams.pinax.network:443 87606376:87714334 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 100 --development-mode

.PHONY: deploy
deploy:
	graph build
	graph deploy --studio clock