use crate::model::{
    NewExpense, NewPayment, Periodicity, add_expense, add_payment, generate_rows, get_entries,
};
use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use rusqlite::{Connection, Error, ffi};
use tabled::Table;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
    Add {
        #[arg(short, long)]
        name: String,

        #[arg(short, long, value_enum)]
        period: Periodicity,

        #[arg(short, long)]
        date: String,
    },
    Pay {
        #[arg(short, long)]
        name: String,

        #[arg(short, long)]
        date: Option<String>,
    },
}

impl Cli {
    pub(crate) fn run(self, conn: &Connection) -> Result<()> {
        match self.command {
            Commands::List => {
                let entries = get_entries(conn).unwrap();
                let rows = generate_rows(&entries);
                let table = Table::new(rows);
                println!("{table}");
            }

            Commands::Add { name, period, date } => {
                let datetime = chrono::DateTime::parse_from_rfc3339(&date);
                let Ok(datetime) = datetime else {
                    return Err(color_eyre::Report::msg(format!(
                        "invalid RFC3339 date: {}. Expecting something like '1996-12-19T16:39:57-08:00'",
                        date
                    )));
                };
                let new_expense = NewExpense {
                    created_at: chrono::Utc::now(),
                    due_date_reference: datetime.to_utc(),
                    name,
                    periodicity: period,
                };
                add_expense(conn, &new_expense)?;
            }
            Commands::Pay { name, date } => {
                let date = if let Some(date) = date {
                    let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(&date) else {
                        return Err(color_eyre::Report::msg(format!(
                            "invalid RFC3339 date: {}. Expecting something like '1996-12-19T16:39:57-08:00'",
                            date
                        )));
                    };

                    datetime.to_utc()
                } else {
                    chrono::Utc::now()
                };

                let new_payment = NewPayment {
                    created_at: chrono::Utc::now(),
                    paid_at: date,
                    expense_name: &name,
                };

                if let Err(Error::SqliteFailure(ffi::Error { extended_code, .. }, _)) =
                    add_payment(conn, &new_payment)
                    && extended_code == 787
                {
                    return Err(color_eyre::Report::msg(format!(
                        "expense with name {} does not exist",
                        name
                    )));
                };
            }
        }

        Ok(())
    }
}
