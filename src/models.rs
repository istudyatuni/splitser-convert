use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ------------- currencies -------------

#[derive(Debug, Deserialize)]
pub struct CurrencyConfig {
    pub subunit_to_unit: u64,
}

// ------------- config -------------

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ExportConfig {
    SingleUser(ExportConfigSingleUser),
    ManyUsers(HashMap<String, String>),
}

#[derive(Debug, Deserialize)]
pub struct ExportConfigSingleUser {
    pub user_id: String,
    pub name: String,
}

// ------------- raw data -------------

#[derive(Debug, Deserialize)]
pub struct ExpensesList {
    pub data: Vec<ExpenseItem>,
}

#[derive(Debug, Deserialize)]
pub struct ExpenseItem {
    pub expense: Expense,
}

#[derive(Debug, Deserialize)]
pub struct Expense {
    pub name: String,
    pub status: ExpenseStatus,
    #[serde(rename = "payed_on")]
    pub date: String,
    // #[serde(flatten)]
    // pub amounts: ExpenseAmounts,
    pub shares: Vec<ExpenseShareItem>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ExpenseStatus {
    Active,
    Deleted,
}

#[derive(Debug, Deserialize)]
pub struct ExpenseAmounts {
    #[serde(rename = "source_amount")]
    pub source: ExpenseAmount,
    #[serde(rename = "amount")]
    pub converted: ExpenseAmount,
}

#[derive(Debug, Deserialize)]
pub struct ExpenseAmount {
    pub currency: String,
    pub fractional: u64,
}

#[derive(Debug, Deserialize)]
pub struct ExpenseShareItem {
    pub share: ExpenseShare,
}

#[derive(Debug, Deserialize)]
pub struct ExpenseShare {
    pub member_id: String,
    #[serde(flatten)]
    pub amounts: ExpenseAmounts,
}

// ------------- exported data -------------

#[derive(Debug, Serialize)]
pub struct ExpenseExport {
    pub name: String,
    pub date: String,

    // pub payed_by: String,
    pub source_currency: String,
    pub source_amount: String,

    pub converted_currency: String,
    pub converted_amount: String,
}
