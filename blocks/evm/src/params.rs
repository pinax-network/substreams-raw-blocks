use std::collections::HashSet;

use substreams::errors::Error;

// Every table produced by `map_events`, i.e. every repeated field of `pinax.evm.v2.Events`.
pub const TABLE_NAMES: [&str; 22] = [
    "blocks",
    "transactions",
    "logs",
    "calls",
    "balance_changes",
    "code_changes",
    "storage_changes",
    "nonce_changes",
    "gas_changes",
    "account_creations",
    "keccak_preimages",
    "access_lists",
    "set_code_authorizations",
    "uncles",
    "system_calls",
    "system_balance_changes",
    "system_code_changes",
    "system_storage_changes",
    "system_nonce_changes",
    "system_gas_changes",
    "system_account_creations",
    "system_keccak_preimages",
];

// Which tables `map_events` emits, driven by the module `params` string.
//
// Grammar (query-string style, `&`-separated):
//   ""                                   -> every table
//   "exclude=storage_changes,gas_changes" -> every table except those
//   "include=blocks,transactions,logs"    -> only those
// Unknown table names are a deterministic error so typos surface immediately.
#[derive(Debug, Clone)]
pub struct Tables {
    enabled: HashSet<&'static str>,
}

impl Tables {
    pub fn all() -> Self {
        Tables {
            enabled: TABLE_NAMES.iter().copied().collect(),
        }
    }

    pub fn from_params(params: &str) -> Result<Self, Error> {
        let mut tables = Tables::all();
        for pair in params.split('&').map(str::trim).filter(|p| !p.is_empty()) {
            let (key, value) = pair.split_once('=').ok_or_else(|| Error::msg(format!("invalid param {pair:?}, expected key=value")))?;
            let names = parse_table_list(value)?;
            match key.trim() {
                "exclude" => {
                    for name in names {
                        tables.enabled.remove(name);
                    }
                }
                "include" => tables.enabled = names.into_iter().collect(),
                other => return Err(Error::msg(format!("unknown param {other:?}, expected `exclude` or `include`"))),
            }
        }
        Ok(tables)
    }

    pub fn has(&self, name: &str) -> bool {
        self.enabled.contains(name)
    }
}

fn parse_table_list(value: &str) -> Result<Vec<&'static str>, Error> {
    value
        .split(',')
        .map(str::trim)
        .filter(|n| !n.is_empty())
        .map(|n| {
            TABLE_NAMES
                .iter()
                .copied()
                .find(|t| *t == n)
                .ok_or_else(|| Error::msg(format!("unknown table {n:?}, expected one of: {}", TABLE_NAMES.join(", "))))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_params_enable_everything() {
        let t = Tables::from_params("").unwrap();
        assert!(TABLE_NAMES.iter().all(|n| t.has(n)));
    }

    #[test]
    fn exclude_removes_tables() {
        let t = Tables::from_params("exclude=storage_changes, system_storage_changes").unwrap();
        assert!(!t.has("storage_changes"));
        assert!(!t.has("system_storage_changes"));
        assert!(t.has("blocks"));
    }

    #[test]
    fn include_keeps_only_listed_tables() {
        let t = Tables::from_params("include=blocks,logs").unwrap();
        assert!(t.has("blocks") && t.has("logs"));
        assert!(!t.has("transactions"));
    }

    #[test]
    fn unknown_table_is_an_error() {
        assert!(Tables::from_params("exclude=traces").is_err());
        assert!(Tables::from_params("foo=bar").is_err());
    }
}
