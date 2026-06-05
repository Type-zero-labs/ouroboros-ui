//! Component tokens — thin per-component override structs.
//!
//! Each component may expose a small token struct (e.g. `ButtonTokens`, `InputTokens`)
//! whose `Default` derives from the [`semantic`](super::semantic) layer, letting a
//! single component be retuned without touching global tokens. Scaffolded in T04;
//! populated as atoms arrive.
