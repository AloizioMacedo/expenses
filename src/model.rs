use chrono::Utc;
use clap::ValueEnum;
use color_eyre::Result;
use rusqlite::{
    Connection, Error, ToSql,
    types::{FromSql, FromSqlResult, ValueRef},
};
use std::{fmt::Display, fs::File, path::PathBuf};
use tabled::Tabled;

pub(crate) fn get_data_path() -> PathBuf {
    let dir_path = std::env::home_dir()
        .expect("should have home dir defined.")
        .join(".expenses");
    _ = std::fs::create_dir_all(&dir_path);

    let file_path = dir_path.join("data.sqlite");
    _ = File::create_new(&file_path); // Err in case file exists, so just ignoring it.

    file_path
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum Periodicity {
    Weekly,
    Biweekly,
    Monthly,
    Bimonthly,
    Trimonthly,
    Quarterly,
    Biannual,
}
impl Display for Periodicity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Periodicity::Weekly => f.write_str("Weekly"),
            Periodicity::Biweekly => f.write_str("Biweekly"),
            Periodicity::Monthly => f.write_str("Monthly"),
            Periodicity::Bimonthly => f.write_str("Bimonthly"),
            Periodicity::Trimonthly => f.write_str("Trimonthly"),
            Periodicity::Quarterly => f.write_str("Quarterly"),
            Periodicity::Biannual => f.write_str("Biannual"),
        }
    }
}

impl FromSql for Periodicity {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value.as_str() {
            Ok("Weekly") => FromSqlResult::Ok(Periodicity::Weekly),
            Ok("Biweekly") => FromSqlResult::Ok(Periodicity::Biweekly),
            Ok("Monthly") => FromSqlResult::Ok(Periodicity::Monthly),
            Ok("Bimonthly") => FromSqlResult::Ok(Periodicity::Bimonthly),
            Ok("Trimonthly") => FromSqlResult::Ok(Periodicity::Trimonthly),
            Ok("Quarterly") => FromSqlResult::Ok(Periodicity::Quarterly),
            Ok("Biannual") => FromSqlResult::Ok(Periodicity::Biannual),
            _ => FromSqlResult::Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

impl ToSql for Periodicity {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        match self {
            Periodicity::Weekly => rusqlite::Result::Ok(rusqlite::types::ToSqlOutput::Borrowed(
                ValueRef::Text("Weekly".as_bytes()),
            )),
            Periodicity::Biweekly => rusqlite::Result::Ok(rusqlite::types::ToSqlOutput::Borrowed(
                ValueRef::Text("Biweekly".as_bytes()),
            )),
            Periodicity::Monthly => rusqlite::Result::Ok(rusqlite::types::ToSqlOutput::Borrowed(
                ValueRef::Text("Monthly".as_bytes()),
            )),

            Periodicity::Bimonthly => rusqlite::Result::Ok(rusqlite::types::ToSqlOutput::Borrowed(
                ValueRef::Text("Bimonthly".as_bytes()),
            )),
            Periodicity::Trimonthly => rusqlite::Result::Ok(
                rusqlite::types::ToSqlOutput::Borrowed(ValueRef::Text("Trimonthly".as_bytes())),
            ),
            Periodicity::Quarterly => rusqlite::Result::Ok(rusqlite::types::ToSqlOutput::Borrowed(
                ValueRef::Text("Quarterly".as_bytes()),
            )),
            Periodicity::Biannual => rusqlite::Result::Ok(rusqlite::types::ToSqlOutput::Borrowed(
                ValueRef::Text("Biannual".as_bytes()),
            )),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct Expense {
    pub(crate) id: i32,
    pub(crate) created_at: chrono::DateTime<Utc>,
    pub(crate) name: String,
    pub(crate) periodicity: Periodicity,
    pub(crate) due_date_reference: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub(crate) struct NewExpense {
    pub(crate) created_at: chrono::DateTime<Utc>,
    pub(crate) name: String,
    pub(crate) periodicity: Periodicity,
    pub(crate) due_date_reference: chrono::DateTime<Utc>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct Payment {
    pub(crate) id: i32,
    pub(crate) created_at: chrono::DateTime<Utc>,
    pub(crate) paid_at: chrono::DateTime<Utc>,
    pub(crate) expense_name: String,
}

#[derive(Debug, Clone)]
pub(crate) struct NewPayment<'a> {
    pub(crate) created_at: chrono::DateTime<Utc>,
    pub(crate) paid_at: chrono::DateTime<Utc>,
    pub(crate) expense_name: &'a str,
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
                 id            INTEGER PRIMARY KEY,
                 created_at    TEXT NOT NULL,
                 paid_at       TEXT NOT NULL,
                 expense_name  TEXT NOT NULL,
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
        "INSERT INTO payment (created_at, paid_at, expense_name) VALUES (?1, ?2, ?3)",
        (&payment.created_at, &payment.paid_at, &payment.expense_name),
    )?;

    Ok(())
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
  p.expense_name AS payment_expense_name
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

#[derive(Tabled)]
pub(crate) struct RowDisplay<'a> {
    expense_name: &'a str,
    last_payment: String,
    periodicity: Periodicity,
}

pub(crate) fn generate_rows<'a>(entries: &'a [(Expense, Option<Payment>)]) -> Vec<RowDisplay<'a>> {
    entries
        .iter()
        .map(|(expense, payment)| RowDisplay {
            expense_name: &expense.name,
            last_payment: payment
                .as_ref()
                .map(|x| x.paid_at.to_rfc2822())
                .unwrap_or("Not paid".to_string()),
            periodicity: expense.periodicity,
        })
        .collect()
}
