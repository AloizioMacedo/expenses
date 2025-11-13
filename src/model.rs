use chrono::Utc;
use clap::ValueEnum;
use rusqlite::{
    ToSql,
    types::{FromSql, FromSqlResult, ValueRef},
};
use std::fmt::Display;

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
