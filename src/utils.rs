use chrono::{Datelike, Days, Local, Months, Utc};
use tabled::Tabled;

use crate::model::{Expense, Payment, Periodicity};

#[derive(Tabled)]
pub(crate) struct RowDisplay<'a> {
    expense_name: &'a str,
    last_payment: String,
    periodicity: Periodicity,
    next_due_date: String,
    days_left: i64,
    is_paid: &'static str,
}

pub(crate) fn get_next_due_date(
    reference: &chrono::DateTime<Utc>,
    periodicity: Periodicity,
) -> chrono::DateTime<Utc> {
    let now = Utc::now();
    let mut reference = *reference;
    if reference > now {
        return reference;
    }

    // Making the loops below O(1) on (now - reference). We could try calculating things and
    // untangling issues like leap days, months having different durations etc,
    // but I don't think it is worth it.
    if reference.year() < now.year() - 1
        && let Some(new_ref) = reference.with_year(now.year())
    {
        reference = new_ref;
    };

    match periodicity {
        Periodicity::Weekly => {
            while reference < now {
                reference = reference.checked_add_days(Days::new(7)).expect("should not be reaching out of bounds for time operations. Are you in the FAR future? o_o");
            }

            reference
        }
        Periodicity::Monthly => {
            while reference < now {
                reference = reference.checked_add_months(Months::new(1)).expect("should not be reaching out of bounds for time operations. Are you in the FAR future? o_o");
            }

            reference
        }
        Periodicity::Bimonthly => {
            while reference < now {
                reference = reference.checked_add_months(Months::new(2)).expect("should not be reaching out of bounds for time operations. Are you in the FAR future? o_o");
            }

            reference
        }
        Periodicity::Trimonthly => {
            while reference < now {
                reference = reference.checked_add_months(Months::new(3)).expect("should not be reaching out of bounds for time operations. Are you in the FAR future? o_o");
            }

            reference
        }
        Periodicity::Quarterly => {
            while reference < now {
                reference = reference.checked_add_months(Months::new(4)).expect("should not be reaching out of bounds for time operations. Are you in the FAR future? o_o");
            }

            reference
        }
        Periodicity::Biannual => {
            while reference < now {
                reference = reference.checked_add_months(Months::new(6)).expect("should not be reaching out of bounds for time operations. Are you in the FAR future? o_o");
            }

            reference
        }
    }
}

pub(crate) fn generate_rows<'a>(entries: &'a [(Expense, Option<Payment>)]) -> Vec<RowDisplay<'a>> {
    entries
        .iter()
        .map(|(expense, payment)| {
            let next_due_date = get_next_due_date(&expense.due_date_reference, expense.periodicity);

            RowDisplay {
                expense_name: &expense.name,
                last_payment: payment
                    .as_ref()
                    .map(|p| p.paid_at.with_timezone(&Local).date_naive().to_string())
                    .unwrap_or("Not paid".to_string()),
                periodicity: expense.periodicity,
                next_due_date: next_due_date.with_timezone(&Local).date_naive().to_string(),
                days_left: next_due_date.signed_duration_since(Utc::now()).num_days(),
                is_paid: payment
                    .as_ref()
                    .map(|p| {
                        if p.due_date_of_expense == next_due_date {
                            "✅"
                        } else {
                            "❌"
                        }
                    })
                    .unwrap_or("❌"),
            }
        })
        .collect()
}
