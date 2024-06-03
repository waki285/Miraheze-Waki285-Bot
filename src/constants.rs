use std::collections::HashMap;

use once_cell::sync::Lazy;

pub const IMPLICIT_GROUPS: [&str; 3] = ["*", "user", "autoconfirmed"];
pub const TEMPORAL_GROUPS: [&str; 4] = ["checkuser", "suppress", "electionadmin", "flood"];
pub const GLOBAL_TEMPORAL_GROUPS: [&str; 1] = ["global-flood"];

pub const REWRITE_GROUPS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    [("trustandsafety", "trust-and-safety")]
        .into_iter()
        .collect()
});

pub static STATUS_PAGE: &str = "User:Waki285-Bot/status";