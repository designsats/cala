#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]

mod id;

pub mod account;
pub mod balance;
pub mod entry;
pub mod journal;
pub mod outbox;
pub mod primitives;
pub mod transaction;
pub mod tx_template;
