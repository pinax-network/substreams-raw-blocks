# Raw EVM blocks -> PostgreSQL via `substreams sink postgres` (Relational Mappings Mode).
#
#   make build                      # cargo build + substreams pack
#   make sink                       # stream every table into $(DSN)
#   make sink MODULE=map_events_no_storage_changes
#   make sink PARAMS="exclude=gas_changes,keccak_preimages"
#   make estimate START=0 STOP=+1000000   # egress with vs. without storage changes
#
# Any Pinax EVM endpoint works: NETWORK=eth|base|bsc|arbitrum-one|polygon|...
NETWORK  ?= base
ENDPOINT ?= $(NETWORK).substreams.pinax.network:443
DSN      ?= postgres://pg:pass@localhost:5433/raw_blocks?sslmode=disable
MODULE   ?= map_events
PARAMS   ?=
START    ?= -1
STOP     ?= 0
SAMPLE   ?= 1
ARGS     ?=

ifneq ($(PARAMS),)
PARAM_FLAG := -p $(MODULE)="$(PARAMS)"
endif

.PHONY: all
all: build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack
	substreams info

.PHONY: protogen
protogen:
	substreams protogen

.PHONY: gui
gui:
	substreams gui substreams.yaml $(MODULE) -e $(ENDPOINT) --network $(NETWORK) -s $(START) -t $(STOP) $(PARAM_FLAG)

.PHONY: run
run:
	substreams run substreams.yaml $(MODULE) -e $(ENDPOINT) --network $(NETWORK) -s $(START) -t $(STOP) -o jsonl $(PARAM_FLAG)

# Create the schema/tables (idempotent), then widen VARCHAR(255) string columns to TEXT
# (see ../../sql/varchar_to_text.sql). Run once before the first `make sink`.
.PHONY: sink-setup
sink-setup:
	substreams sink postgres setup substreams.yaml $(MODULE) --dsn "$(DSN)"
	psql "$(DSN)" -f ../../sql/varchar_to_text.sql

# Backfill + live. Constraints are created once the stream reaches chain HEAD
# (--apply-constraints=auto); use `make sink-constraints` for a manual pass.
.PHONY: sink
sink:
	substreams sink postgres substreams.yaml $(MODULE) --dsn "$(DSN)" -e $(ENDPOINT) --network $(NETWORK) -s $(START) -t $(STOP) $(PARAM_FLAG) $(ARGS)

.PHONY: sink-no-storage-changes
sink-no-storage-changes:
	$(MAKE) sink MODULE=map_events_no_storage_changes

.PHONY: sink-constraints
sink-constraints:
	substreams sink postgres constraints apply substreams.yaml $(MODULE) --dsn "$(DSN)"

# Egress estimates of the raw Firehose block, with and without storage changes.
.PHONY: estimate
estimate:
	substreams estimate substreams.yaml map_block -e $(ENDPOINT) --network $(NETWORK) -s $(START) -t $(STOP) --sample-percentage $(SAMPLE) --yes
	substreams estimate substreams.yaml map_block_no_storage_changes -e $(ENDPOINT) --network $(NETWORK) -s $(START) -t $(STOP) --sample-percentage $(SAMPLE) --yes

# Egress estimate of the Firehose block with any families removed, e.g.
#   make estimate-trimmed NETWORK=eth START=0 STOP=25890616 EXCLUDE=storage_changes,gas_changes
EXCLUDE ?= storage_changes
.PHONY: estimate-trimmed
estimate-trimmed:
	substreams estimate substreams.yaml map_block_trimmed -e $(ENDPOINT) --network $(NETWORK) -s $(START) -t $(STOP) --sample-percentage $(SAMPLE) --yes -p map_block_trimmed="exclude=$(EXCLUDE)"

# Per-block byte breakdown (storage changes, keccak preimages, gas changes, ...).
.PHONY: sizes
sizes:
	substreams run substreams.yaml map_block_sizes -e $(ENDPOINT) --network $(NETWORK) -s $(START) -t $(STOP) -o jsonl
