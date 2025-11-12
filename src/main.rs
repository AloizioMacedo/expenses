use anyhow::Result;
use chrono::Utc;
use rusqlite::Connection;

mod model;

use crate::model::{
    NewExpense, NewPayment, add_expense, add_payment, create_tables, get_data_path, get_entries,
};

fn main() -> Result<()> {
    let conn = Connection::open(get_data_path())?;
    _ = create_tables(&conn);

    add_expense(
        &conn,
        &NewExpense {
            created_at: Utc::now(),
            name: "Test".to_string(),
            periodicity: model::Periodicity::Monthly,
            due_date_reference: Utc::now(),
        },
    )?;
    add_payment(
        &conn,
        &NewPayment {
            created_at: Utc::now(),
            expense_name: "Test".to_string(),
            paid_at: Utc::now(),
        },
    )?;

    for expense in get_entries(&conn).unwrap() {
        println!("expense: {expense:?}");
    }

    Ok(())
}
