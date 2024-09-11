.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack
	graph build
	substreams info

.PHONY: deploy
deploy:
	graph create --node http://localhost:8020 eos_transactions
	graph deploy --node=http://localhost:8020 eos_transactions

.PHONY: publish
publish:
	graph publish --subgraph-id 2RNdhL5p62dGN5UqKtsSEhYZiTJbFcuuhzk9qRJj8QeU

.PHONY: gui
gui:
	substreams gui -e eos.substreams.pinax.network:443 graph_out -s -1