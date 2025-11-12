use anyhow::Result;
use chrono::Utc;
use rusqlite::{
    Connection, ToSql,
    types::{FromSql, FromSqlResult, ValueRef},
};
use std::{fs::File, path::PathBuf};

pub(crate) fn get_data_path() -> PathBuf {
    let dir_path = std::env::home_dir()
        .expect("should have home dir defined.")
        .join(".expenses");
    _ = std::fs::create_dir_all(&dir_path);

    let file_path = dir_path.join("data.sqlite");
    _ = File::create_new(&file_path); // Err in case file exists, so just ignoring it.

    file_path
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Periodicity {
    Weekly,
    Biweekly,
    Monthly,
    Bimonthly,
    Trimonthly,
    Quarterly,
    Biannual,
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

impl From<Periodicity> for &'static str {
    fn from(value: Periodicity) -> Self {
        match value {
            Periodicity::Weekly => "Weekly",
            Periodicity::Biweekly => "Biweekly",
            Periodicity::Monthly => "Monthly",
            Periodicity::Bimonthly => "Bimonthly",
            Periodicity::Trimonthly => "Trimonthly",
            Periodicity::Quarterly => "Quarterly",
            Periodicity::Biannual => "Biannual",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Expense {
    pub(crate) id: i32,
    pub(crate) created_at: chrono::DateTime<Utc>,
    pub(crate) name: String,
    pub(crate) periodicity: Periodicity,
    pub(crate) last_paid_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub(crate) struct NewExpense {
    pub(crate) created_at: chrono::DateTime<Utc>,
    pub(crate) name: String,
    pub(crate) periodicity: Periodicity,
    pub(crate) last_paid_at: Option<chrono::DateTime<Utc>>,
}

pub(crate) fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE expense (
                 id            INTEGER PRIMARY KEY,
                 created_at    TEXT NOT NULL,
                 name          TEXT NOT NULL,
                 periodicity   TEXT NOT NULL,
                 last_paid_at  TEXT
              )",
        (),
    )?;

    Ok(())
}

pub(crate) fn add_entry(conn: &Connection, expense: NewExpense) -> Result<()> {
    conn.execute(
        "INSERT INTO expense (created_at, name, periodicity, last_paid_at) VALUES (?1, ?2, ?3, ?4)",
        (
            &expense.created_at,
            &expense.name,
            &expense.periodicity,
            &expense.last_paid_at,
        ),
    )?;

    Ok(())
}

pub(crate) fn get_entries(conn: &Connection) -> Result<Vec<Expense>> {
    let mut stmt =
        conn.prepare("SELECT id, created_at, name, periodicity, last_paid_at FROM expense")?;
    let expenses = stmt.query_map([], |row| {
        Ok(Expense {
            id: row.get(0)?,
            created_at: row.get(1)?,
            name: row.get(2)?,
            periodicity: row.get(3)?,
            last_paid_at: row.get(4)?,
        })
    })?;

    let mut expenses_to_return = Vec::new();

    for expense in expenses {
        let expense = expense?;

        expenses_to_return.push(expense);
    }

    Ok(expenses_to_return)
}
