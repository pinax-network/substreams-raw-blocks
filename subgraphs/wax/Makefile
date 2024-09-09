.PHONY: all
all:
	make build
	make deploy

.PHONY: build
build:
	graph build

.PHONY: deploy
deploy:
	graph create --node http://localhost:8020 wax_transactions
	graph deploy --node=http://localhost:8020 wax_transactions

.PHONY: publish
publish:
	graph publish --subgraph-id 4bAe7NA8b6J14ZfZr3TXfzzjjSoGECTFuqv7CwnK1zzg