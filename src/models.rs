use serde::Deserialize;

// ------------- config -------------

#[derive(Debug, Deserialize)]
pub struct ExportConfig {
    pub user_id: String,
}

// ------------- raw data -------------

#[derive(Debug, Deserialize)]
pub struct MembersList {
    pub data: Vec<MemberItem>,
}

#[derive(Debug, Deserialize)]
pub struct MemberItem {
    pub member: Member,
}

#[derive(Debug, Deserialize)]
pub struct Member {
    #[serde(rename = "user_id")]
    pub id: String,
    pub nickname: String,
}

#[derive(Debug, Deserialize)]
pub struct ExpencesList {
    pub data: Vec<ExpenceItem>,
}

#[derive(Debug, Deserialize)]
pub struct ExpenceItem {
    pub expense: Expence,
}

#[derive(Debug, Deserialize)]
pub struct Expence {
    pub name: String,
    pub status: ExpenceStatus,
    pub payed_on: String,
    #[serde(flatten)]
    pub amounts: ExpenceAmounts,
    pub shares: Vec<ExpenceShareItem>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ExpenceStatus {
    Active,
    Deleted,
}

#[derive(Debug, Deserialize)]
pub struct ExpenceAmounts {
    #[serde(rename = "source_amount")]
    pub source: ExpenceAmount,
    #[serde(rename = "amount")]
    pub converted: ExpenceAmount,
}

#[derive(Debug, Deserialize)]
pub struct ExpenceAmount {
    pub currency: String,
    pub fractional: u64,
}

#[derive(Debug, Deserialize)]
pub struct ExpenceShareItem {
    pub share: ExpenceShare,
}

#[derive(Debug, Deserialize)]
pub struct ExpenceShare {
    pub member_id: String,
    #[serde(flatten)]
    pub amounts: ExpenceAmounts,
}

// ------------- exported data -------------
