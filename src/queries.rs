use std::{fs::File, path::PathBuf};

use rusqlite::{Connection, Error, Result};

use crate::model::{Expense, NewExpense, NewPayment, Payment};

pub(crate) fn get_data_path() -> PathBuf {
    let dir_path = std::env::home_dir()
        .expect("should have home dir defined.")
        .join(".expenses");
    _ = std::fs::create_dir_all(&dir_path);

    let file_path = dir_path.join("data.sqlite");
    _ = File::create_new(&file_path); // Err in case file exists, so just ignoring it.

    file_path
}

pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE expense (
                 id                  INTEGER PRIMARY KEY,
                 created_at          TEXT NOT NULL,
                 name                TEXT UNIQUE NOT NULL,
                 periodicity         TEXT NOT NULL,
                 due_date_reference  TEXT NOT NULL
              )",
        (),
    )?;
    conn.execute(
    "CREATE TABLE payment (
                 id                  INTEGER PRIMARY KEY,
                 created_at          TEXT NOT NULL,
                 paid_at             TEXT NOT NULL,
                 expense_name        TEXT NOT NULL,
                 due_date_of_expense TEXT NOT NULL,
                 FOREIGN KEY (expense_name) REFERENCES expense(name) ON DELETE CASCADE ON UPDATE CASCADE
              )",
    (),
)?;

    Ok(())
}

pub(crate) fn add_expense(conn: &Connection, expense: &NewExpense) -> Result<()> {
    conn.execute(
    "INSERT INTO expense (created_at, name, periodicity, due_date_reference) VALUES (?1, ?2, ?3, ?4)",
    (
        &expense.created_at,
        &expense.name,
        &expense.periodicity,
        &expense.due_date_reference,
    ),
)?;

    Ok(())
}

pub(crate) fn add_payment(conn: &Connection, payment: &NewPayment) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO payment (created_at, paid_at, expense_name, due_date_of_expense) VALUES (?1, ?2, ?3, ?4)",
        (
            &payment.created_at,
            &payment.paid_at,
            &payment.expense_name,
            &payment.due_date_of_expense,
        ),
    )?;

    Ok(())
}

pub(crate) fn delete_expense(conn: &Connection, name: &str) -> Result<()> {
    conn.execute("DELETE FROM expense WHERE expense.name = ?1", (name,))?;

    Ok(())
}

pub(crate) fn get_expense_by_name(conn: &Connection, name: &str) -> Result<Option<Expense>> {
    let mut stmt = conn.prepare("SELECT id, created_at, due_date_reference, name, periodicity FROM expense WHERE expense.name = ?1")?;

    stmt.query_map([name], |row| {
        Ok(Expense {
            id: row.get(0)?,
            created_at: row.get(1)?,
            due_date_reference: row.get(2)?,
            name: row.get(3)?,
            periodicity: row.get(4)?,
        })
    })
    .map(|x| x.flatten().next())
}

pub(crate) fn get_entries(conn: &Connection) -> Result<Vec<(Expense, Option<Payment>)>> {
    let mut stmt = conn.prepare(
        "SELECT 
  e.id AS expense_id,
  e.created_at AS expense_created_at,
  e.name AS expense_name,
  e.periodicity,
  e.due_date_reference,
  p.id AS payment_id,
  p.created_at AS payment_created_at,
  p.paid_at,
  p.expense_name AS payment_expense_name,
  p.due_date_of_expense AS payment_due_date_of_expense
FROM expense e
LEFT JOIN (
  SELECT p1.*
  FROM payment p1
  INNER JOIN (
    SELECT expense_name, MAX(paid_at) AS latest_paid_at
    FROM payment
    GROUP BY expense_name
  ) p2 ON p1.expense_name = p2.expense_name
        AND p1.paid_at = p2.latest_paid_at
) p ON e.name = p.expense_name;",
    )?;
    let expenses = stmt.query_map([], |row| {
        let expense = Expense {
            id: row.get(0)?,
            created_at: row.get(1)?,
            name: row.get(2)?,
            periodicity: row.get(3)?,
            due_date_reference: row.get(4)?,
        };
        let payment_id: Option<i32> = row.get(5)?;
        if payment_id.is_some() {
            Ok((
                expense,
                Some(Payment {
                    id: row.get(5)?,
                    created_at: row.get(6)?,
                    paid_at: row.get(7)?,
                    expense_name: row.get(8)?,
                    due_date_of_expense: row.get(9)?,
                }),
            ))
        } else {
            Ok((expense, None))
        }
    })?;

    let mut expenses_to_return = Vec::new();

    for expense in expenses {
        let expense = expense?;

        expenses_to_return.push(expense);
    }

    Ok(expenses_to_return)
}
