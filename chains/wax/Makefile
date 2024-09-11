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
	graph create --node http://localhost:8020 wax_transactions
	graph deploy --node=http://localhost:8020 wax_transactions

.PHONY: publish
publish:
	graph publish --subgraph-id 4bAe7NA8b6J14ZfZr3TXfzzjjSoGECTFuqv7CwnK1zzg

.PHONY: gui
gui:
	substreams gui -e wax.substreams.pinax.network:443 graph_out -s -1