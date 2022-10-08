use super::pattern::Pattern;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ActionOpts {
    #[serde(flatten)]
    pub action: Action,
}

#[derive(Debug)]
pub enum ActionError {
    NotImplemented,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Action {
    #[serde(rename = "replaceDesc")]
    ReplaceDescription {
        replace: Pattern,
        with: String,
    },
    ApplyTag {
        tag: String,
    },
    #[cfg(feature = "transaction")]
    AddItems {
        items: Vec<crate::statements::transaction::TransactionItem>,
    },
}
