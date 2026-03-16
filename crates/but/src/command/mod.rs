//! A place for each command, i.e. `but foo` as `pub mod foo` here.
#[cfg(feature = "legacy")]
pub mod legacy;

pub mod alias;
pub mod branch;
pub mod commit;
pub mod completions;
#[cfg(feature = "native")]
pub mod config;
pub mod eval_hook;
pub(crate) mod git_config;
pub mod gui;
pub mod help;
pub mod onboarding;
pub mod push;
#[cfg(feature = "native")]
pub mod skill;
pub mod update;
