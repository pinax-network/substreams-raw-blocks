NETWORK ?= eth-cl
ENDPOINT ?= $(NETWORK).substreams.pinax.network:443
HASH = $(shell substreams info --json | jq -r '.modules[0].hash')

.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack
	substreams graph
	substreams info

.PHONY: protogen
protogen:
	substreams protogen

.PHONY: gui
gui:
	substreams gui . -e "$(NETWORK).substreams.pinax.network:443" map_events -s 9000000 -t 0 --network $(NETWORK)

# ---------------------------------------------------------------------------
# PostgreSQL via `substreams sink postgres` (Relational Mappings Mode).
# NOTE: this package's schema is not yet aligned with blocks/evm (see
# https://github.com/pinax-network/substreams-raw-blocks/issues/33); tables are
# named after the output message types until then.
# ---------------------------------------------------------------------------
DSN    ?= postgres://pg:pass@localhost:5433/raw_blocks?sslmode=disable
MODULE ?= map_events
START  ?= -1
STOP   ?= 0
ARGS   ?=

.PHONY: sink-setup
sink-setup:
	substreams sink postgres setup substreams.yaml $(MODULE) --dsn "$(DSN)"
	psql "$(DSN)" -f ../../sql/varchar_to_text.sql

.PHONY: sink
sink:
	substreams sink postgres substreams.yaml $(MODULE) --dsn "$(DSN)" -e $(ENDPOINT) --network $(NETWORK) -s $(START) -t $(STOP) $(ARGS)

.PHONY: sink-constraints
sink-constraints:
	substreams sink postgres constraints apply substreams.yaml $(MODULE) --dsn "$(DSN)"
