use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use models::*;

mod models;

fn main() -> Result<()> {
    let export_folder = PathBuf::from("data/export");
    if !export_folder.exists() {
        std::fs::create_dir(&export_folder).context("creating export folder")?;
    }

    let config = std::fs::read_to_string("data/config.json").context("reading config file")?;
    let config: ExportConfig = serde_json::from_str(&config).context("parsing config json")?;

    let currencies =
        std::fs::read_to_string("data/currencies.json").context("reading currencies file")?;
    let currencies: HashMap<String, CurrencyConfig> =
        serde_json::from_str(&currencies).context("parsing currencies json")?;
    // how much to shift decimal separator
    let currencies_sub_shift: HashMap<_, _> = currencies
        .into_iter()
        .map(|(name, config)| {
            (
                name,
                // count number of zeros in number, imitate log10
                config
                    .subunit_to_unit
                    .to_string()
                    .chars()
                    .filter(|&c| c == '0')
                    .count(),
            )
        })
        .collect();

    let expenses =
        std::fs::read_to_string("data/expenses.json").context("reading expenses file")?;
    let expenses: ExpensesList =
        serde_json::from_str(&expenses).context("parsing expenses json")?;
    let expenses: Vec<_> = expenses
        .data
        .into_iter()
        .filter(|e| e.expense.status == ExpenseStatus::Active)
        .map(|e| e.expense)
        .collect();

    match config {
        ExportConfig::SingleUser(config) => export_member(
            &config.user_id,
            &export_folder.join(&config.name).with_extension("csv"),
            &currencies_sub_shift,
            &expenses,
        )
        .context("exporting member data")?,
        ExportConfig::ManyUsers(map) => {
            for (user_id, name) in &map {
                export_member(
                    user_id,
                    &export_folder.join(name).with_extension("csv"),
                    &currencies_sub_shift,
                    &expenses,
                )
                .with_context(|| format!("exporting member data for {name} ({user_id})"))?
            }
        }
    }

    Ok(())
}

fn export_member(
    user_id: &str,
    path: &Path,
    currencies_sub_shift: &HashMap<String, usize>,
    expenses: &[Expense],
) -> Result<()> {
    let mut writer = csv::Writer::from_path(path).context("initializing csv writer")?;
    let mut written = 0;
    for expense in expenses {
        let Some(share) = expense.shares.iter().find(|s| s.share.member_id == user_id) else {
            continue;
        };
        let share = &share.share.amounts;

        writer
            .serialize(ExpenseExport {
                name: expense.name.to_owned(),
                date: expense.date.to_owned(),
                source_currency: share.source.currency.clone(),
                source_amount: format_amount(
                    *currencies_sub_shift
                        .get(&share.source.currency)
                        .expect("currency should be known"),
                    share.source.fractional,
                ),
                converted_currency: share.converted.currency.clone(),
                converted_amount: format_amount(
                    *currencies_sub_shift
                        .get(&share.converted.currency)
                        .expect("currency should be known"),
                    share.converted.fractional,
                ),
            })
            .context("serializing expense for csv")?;
        written += 1;
    }
    writer.flush().context("flushing csv writer")?;

    eprintln!("written {written} expenses");

    Ok(())
}

fn format_amount(period_from_right_side: usize, amount: u64) -> String {
    let mut amount = amount.to_string();
    if period_from_right_side == 0 {
        return amount;
    }

    while amount.len() <= period_from_right_side {
        amount.insert(0, '0');
    }

    // using period instead of dot because google spreadsheets convert some values to dates
    amount.insert(amount.len() - period_from_right_side, ',');
    amount
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_currency() {
        let table = [
            (2, 1200, "12,00"),
            (2, 12, "0,12"),
            (2, 1200, "12,00"),
            (0, 1200, "1200"),
            (8, 1200, "0,00001200"),
        ];
        for (period_from_right_side, amount, expected) in table {
            let res = format_amount(period_from_right_side, amount);
            assert_eq!(
                expected, res,
                "format_currency({period_from_right_side}, {amount})"
            );
        }
    }
}
