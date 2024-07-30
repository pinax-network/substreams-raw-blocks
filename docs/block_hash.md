To compute the Ethereum block hash, you'll need to RLP-encode the block header and then apply the Keccak-256 hash function to the encoded data. The block header consists of multiple fields that must be included in the correct order, as specified in the protocol. Here's a detailed explanation of how to do this:

### Step-by-Step Process

1. **Collect Block Header Fields**: Gather all the necessary fields from the block header.
2. **RLP Encode the Header**: Encode the collected fields using RLP encoding.
3. **Compute the Keccak-256 Hash**: Apply the Keccak-256 hash function to the RLP-encoded header.

### Detailed Steps

#### 1. Collect Block Header Fields

The block header contains the following fields:

- `parent_hash`: The Keccak-256 hash of the parent block's header.
- `uncle_hash`: The Keccak-256 hash of the uncles list.
- `coinbase`: The address of the miner.
- `state_root`: The root hash of the state trie.
- `transactions_root`: The root hash of the transactions trie.
- `receipt_root`: The root hash of the receipts trie.
- `logs_bloom`: The bloom filter for logs.
- `difficulty`: The difficulty level of the block.
- `number`: The block number.
- `gas_limit`: The maximum amount of gas allowed in the block.
- `gas_used`: The amount of gas used by the block.
- `timestamp`: The time at which the block was mined.
- `extra_data`: Extra data included by the miner.
- `mix_hash`: A hash used in the proof-of-work algorithm.
- `nonce`: A value used in the proof-of-work algorithm.
- `base_fee_per_gas`: The base fee per gas (included only if the London fork is active).
- `withdrawals_root`: The root hash of the withdrawals list (included only if the Shanghai fork is active).
- `blob_gas_used`: The amount of blob gas used (included only if the Cancun fork is active).
- `excess_blob_gas`: The excess blob gas (included only if the Cancun fork is active).
- `parent_beacon_root`: The root of the parent beacon (included only if the Cancun fork is active).

#### 2. RLP Encode the Header

Use the RLP encoding to encode the block header fields. The RLP encoding must follow the exact order of the fields as listed above.

```python
from eth_utils import keccak, to_bytes
import rlp

# Define block header fields
parent_hash = to_bytes(hexstr="0x...")
uncle_hash = to_bytes(hexstr="0x...")
coinbase = to_bytes(hexstr="0x...")
state_root = to_bytes(hexstr="0x...")
transactions_root = to_bytes(hexstr="0x...")
receipt_root = to_bytes(hexstr="0x...")
logs_bloom = to_bytes(hexstr="0x...")
difficulty = 0x...
number = 0x...
gas_limit = 0x...
gas_used = 0x...
timestamp = 0x...
extra_data = to_bytes(hexstr="0x...")
mix_hash = to_bytes(hexstr="0x...")
nonce = to_bytes(hexstr="0x...")
base_fee_per_gas = 0x...  # Include only if London fork is active
withdrawals_root = to_bytes(hexstr="0x...")  # Include only if Shanghai fork is active
blob_gas_used = 0x...  # Include only if Cancun fork is active
excess_blob_gas = 0x...  # Include only if Cancun fork is active
parent_beacon_root = to_bytes(hexstr="0x...")  # Include only if Cancun fork is active

# Create the block header as a list of fields
block_header = [
    parent_hash,
    uncle_hash,
    coinbase,
    state_root,
    transactions_root,
    receipt_root,
    logs_bloom,
    difficulty,
    number,
    gas_limit,
    gas_used,
    timestamp,
    extra_data,
    mix_hash,
    nonce,
]

# Append fork-specific fields if applicable
if london_fork_active:
    block_header.append(base_fee_per_gas)
if shanghai_fork_active:
    block_header.append(withdrawals_root)
if cancun_fork_active:
    block_header.append(blob_gas_used)
    block_header.append(excess_blob_gas)
    block_header.append(parent_beacon_root)

# RLP encode the block header
rlp_encoded_header = rlp.encode(block_header)
```

#### 3. Compute the Keccak-256 Hash

Apply the Keccak-256 hash function to the RLP-encoded block header to get the block hash.

```python
# Compute the block hash
block_hash = keccak(rlp_encoded_header)
print(f'Block Hash: {block_hash.hex()}')
```

### Full Example in Python

Here's a full example of computing the block hash:

```python
from eth_utils import keccak, to_bytes
import rlp

# Define block header fields
parent_hash = to_bytes(hexstr="0x...")
uncle_hash = to_bytes(hexstr="0x...")
coinbase = to_bytes(hexstr="0x...")
state_root = to_bytes(hexstr="0x...")
transactions_root = to_bytes(hexstr="0x...")
receipt_root = to_bytes(hexstr="0x...")
logs_bloom = to_bytes(hexstr="0x...")
difficulty = 0x...
number = 0x...
gas_limit = 0x...
gas_used = 0x...
timestamp = 0x...
extra_data = to_bytes(hexstr="0x...")
mix_hash = to_bytes(hexstr="0x...")
nonce = to_bytes(hexstr="0x...")
base_fee_per_gas = 0x...  # Include only if London fork is active
withdrawals_root = to_bytes(hexstr="0x...")  # Include only if Shanghai fork is active
blob_gas_used = 0x...  # Include only if Cancun fork is active
excess_blob_gas = 0x...  # Include only if Cancun fork is active
parent_beacon_root = to_bytes(hexstr="0x...")  # Include only if Cancun fork is active

# Determine which forks are active
london_fork_active = True
shanghai_fork_active = False
cancun_fork_active = False

# Create the block header as a list of fields
block_header = [
    parent_hash,
    uncle_hash,
    coinbase,
    state_root,
    transactions_root,
    receipt_root,
    logs_bloom,
    difficulty,
    number,
    gas_limit,
    gas_used,
    timestamp,
    extra_data,
    mix_hash,
    nonce,
]

# Append fork-specific fields if applicable
if london_fork_active:
    block_header.append(base_fee_per_gas)
if shanghai_fork_active:
    block_header.append(withdrawals_root)
if cancun_fork_active:
    block_header.append(blob_gas_used)
    block_header.append(excess_blob_gas)
    block_header.append(parent_beacon_root)

# RLP encode the block header
rlp_encoded_header = rlp.encode(block_header)

# Compute the block hash
block_hash = keccak(rlp_encoded_header)
print(f'Block Hash: {block_hash.hex()}')
```

This code should correctly compute the block hash for an Ethereum block, taking into account the necessary fields and any active forks. Ensure that you have the `eth_utils` and `rlp` libraries installed in your Python environment to run this code.
