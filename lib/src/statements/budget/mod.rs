pub mod quota;

use self::quota::Quota;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Budget {
    pub tag: String,
    pub quota: Quota,
    #[serde(default)]
    pub reserved: bool,
}
