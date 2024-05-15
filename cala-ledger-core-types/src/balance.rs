use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::primitives::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BalanceValues {
    pub journal_id: JournalId,
    pub account_id: AccountId,
    pub entry_id: EntryId,
    pub currency: Currency,
    pub settled_dr_balance: Decimal,
    pub settled_cr_balance: Decimal,
    pub settled_entry_id: EntryId,
    pub settled_modified_at: DateTime<Utc>,
    pub pending_dr_balance: Decimal,
    pub pending_cr_balance: Decimal,
    pub pending_entry_id: EntryId,
    pub pending_modified_at: DateTime<Utc>,
    pub encumbered_dr_balance: Decimal,
    pub encumbered_cr_balance: Decimal,
    pub encumbered_entry_id: EntryId,
    pub encumbered_modified_at: DateTime<Utc>,
}
