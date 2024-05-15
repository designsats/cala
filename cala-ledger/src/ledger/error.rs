use thiserror::Error;

use crate::{
    account::error::AccountError,
    entry::error::EntryError,
    journal::error::JournalError,
    outbox::{error::OutboxError, server::error::OutboxServerError},
    transaction::error::TransactionError,
    tx_template::error::TxTemplateError,
};

#[derive(Error, Debug)]
pub enum LedgerError {
    #[error("LedgerError - Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("LedgerError - Migrate: {0}")]
    SqlxMigrate(#[from] sqlx::migrate::MigrateError),
    #[error("LedgerError - Config: {0}")]
    ConfigError(String),
    #[error("LedgerError - Outbox: {0}")]
    Outbox(#[from] OutboxError),
    #[error("LedgerError - OutboxServer: {0}")]
    OutboxServer(#[from] OutboxServerError),
    #[error("LedgerError - AccountError: {0}")]
    AccountError(#[from] AccountError),
    #[error("LedgerError - JournalError: {0}")]
    JournalError(#[from] JournalError),
    #[error("LedgerError - TxTemplateError: {0}")]
    TxTemplateError(#[from] TxTemplateError),
    #[error("LedgerError - TransactionError: {0}")]
    TransactionError(#[from] TransactionError),
    #[error("LedgerError - EntryError: {0}")]
    EntryError(#[from] EntryError),
}
