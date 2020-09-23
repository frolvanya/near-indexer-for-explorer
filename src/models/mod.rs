use std::env;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;

pub mod enums;
// pub mod access_keys;
// pub mod accounts;
pub mod blocks;
pub mod chunks;
pub mod execution_outcomes;
pub mod receipts;
pub mod transactions;

// pub use access_keys::AccessKey;
// pub use accounts::Account;
pub use blocks::Block;
pub use chunks::Chunk;
pub use execution_outcomes::{ExecutionOutcome, ExecutionOutcomeReceipt};
pub use receipts::{
    Receipt, ReceiptAction, ReceiptActionAction, ReceiptActionInputData, ReceiptActionOutputData,
    ReceiptData,
};
pub use transactions::{Transaction, TransactionAction};

pub(crate) fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| panic!("DATABASE_URL must be set in .env file"));
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    Pool::builder()
        .build(manager)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
