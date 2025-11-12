use anyhow::Result;
use chrono::Utc;
use rusqlite::Connection;

mod utils;

use crate::utils::{NewExpense, add_entry, create_table, get_data_path, get_entries};

fn main() -> Result<()> {
    let conn = Connection::open(get_data_path())?;

    _ = create_table(&conn);
    add_entry(
        &conn,
        NewExpense {
            created_at: Utc::now(),
            name: "Test".to_string(),
            periodicity: utils::Periodicity::Monthly,
            last_paid_at: None,
        },
    )?;
    println!("Hello, world!");

    if let Ok(entries) = get_entries(&conn) {
        for expense in entries {
            println!("expense: {expense:?}");
        }
    }

    Ok(())
}
