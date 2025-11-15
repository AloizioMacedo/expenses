use crate::model::{NewExpense, NewPayment, Periodicity};
use crate::queries::{add_expense, add_payment, delete_expense, get_entries, get_expense_by_name};
use crate::utils::{generate_rows, get_next_due_date};

use chrono::{Datelike, Local, NaiveTime};
use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use rusqlite::{Connection, Error, ffi};
use tabled::Table;

/// Expenses tracker
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lists all expenses
    List,
    /// Adds an expense
    Add {
        /// Name of the expense. Will be used as an identifier
        name: String,

        /// Periodicity of the expense
        #[arg(short, long, value_enum, default_value = "monthly")]
        period: Periodicity,

        /// Date when to pay the expense. Will be used as reference for future payments. Should be in %Y-%m-%d format
        #[arg(short, long)]
        date: String,
    },
    /// Registers a payment to an expense
    Pay {
        /// Name of the expense to pay
        name: String,

        /// When the expense was paid. If not specified, current time is assumed
        #[arg(short, long)]
        date: Option<String>,
    },
    /// Deletes an expense
    Delete {
        /// Name of the expense to delete
        name: String,
    },
}

impl Cli {
    pub(crate) fn run(&self, conn: &Connection) -> Result<()> {
        match &self.command {
            Commands::List => {
                let entries = get_entries(conn).unwrap();
                let rows = generate_rows(&entries);
                let table = Table::new(rows);
                println!("{table}");
            }
            Commands::Add { name, period, date } => {
                let naive_date = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d");
                let Ok(naive_date) = naive_date else {
                    return Err(color_eyre::Report::msg(format!(
                        "invalid date: {}. Expecting something like '1996-12-19'",
                        date
                    )));
                };
                if naive_date.day() > 28 && !matches!(period, Periodicity::Weekly) {
                    return Err(color_eyre::Report::msg(
                        "please choose a day smaller than 29 when using this period",
                    ));
                }
                let naive_datetime = chrono::NaiveDateTime::new(
                    naive_date,
                    NaiveTime::from_hms_opt(0, 0, 0).expect("arguments are valid"),
                );

                let new_expense = NewExpense {
                    created_at: chrono::Utc::now(),
                    due_date_reference: naive_datetime.and_local_timezone(Local).unwrap().to_utc(),
                    name,
                    periodicity: *period,
                };
                add_expense(conn, &new_expense)?;
            }
            Commands::Pay { name, date } => {
                let date = if let Some(date) = date {
                    let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(date) else {
                        return Err(color_eyre::Report::msg(format!(
                            "invalid RFC3339 date: {}. Expecting something like '1996-12-19T16:39:57-08:00'",
                            date
                        )));
                    };

                    datetime.to_utc()
                } else {
                    chrono::Utc::now()
                };
                let expense = get_expense_by_name(conn, name)?;
                let Some(expense) = expense else {
                    return Err(color_eyre::Report::msg(format!(
                        "expense with name {} does not exist",
                        name
                    )));
                };
                let next_due_date =
                    get_next_due_date(&expense.due_date_reference, expense.periodicity);

                let new_payment = NewPayment {
                    created_at: chrono::Utc::now(),
                    paid_at: date,
                    expense_name: name,
                    due_date_of_expense: next_due_date,
                };

                let add_payment_result = add_payment(conn, &new_payment);

                if let Err(Error::SqliteFailure(ffi::Error { extended_code, .. }, _)) =
                    add_payment_result
                    && extended_code == 787
                {
                    return Err(color_eyre::Report::msg(format!(
                        "expense with name {} does not exist",
                        name
                    )));
                }

                add_payment_result?;
            }
            Commands::Delete { name } => delete_expense(conn, name)?,
        }

        Ok(())
    }
}
