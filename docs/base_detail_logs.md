When dealing with `logs` for `DetailLevel: BASE` (RPC Poller) vs. `DetailLevel: EXTENDED` (Full Blocks)

**BASE**
- Failed Transactions do not produce `logs`
- Successful Transactions produces logs in the `transaction.receipt`

**EXTENDED**
- Failed Transactions produces `calls` which includes `logs`
- Successful Transactions produces logs in the `calls.logs` & `transaction.receipt` (need to deduplicate this)

Which means for **BASE**, there are no `logs` that are failed transactions and for **EXTENDED** it can include logs which are from failed  transaction

`tx_success` field will be set as `true/false`, always `true` for `BASE` block detail

Transaction to test this with:
https://etherscan.io/tx/0x5d683ba4c3e27fb59a8f12cd414992cd8a5e1ec111210bceccd879f3d89aa2bc