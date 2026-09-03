NETWORK ?= cosmoshub
ENDPOINT ?= $(NETWORK).substreams.pinax.network:443
.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack
	substreams info
	substreams graph

.PHONY: gui
gui:
	substreams gui substreams.yaml -e $(ENDPOINT) map_events -s -1

.PHONY: protogen
protogen:
	substreams protogen

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
	substreams sink postgres substreams.yaml $(MODULE) --dsn "$(DSN)" -e $(ENDPOINT) -s $(START) -t $(STOP) $(ARGS)

.PHONY: sink-constraints
sink-constraints:
	substreams sink postgres constraints apply substreams.yaml $(MODULE) --dsn "$(DSN)"
