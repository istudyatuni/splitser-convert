#![expect(unused)]

use std::collections::HashMap;

use anyhow::{Context, Result};

use models::*;

mod models;

fn main() -> Result<()> {
    let config = std::fs::read_to_string("data/config.json").context("reading config file")?;
    let config: ExportConfig = serde_json::from_str(&config).context("parsing config json")?;
    let target_user_id = config.user_id;

    let members = std::fs::read_to_string("data/members.json").context("reading members file")?;
    let members: MembersList = serde_json::from_str(&members).context("parsing members json")?;
    let members: HashMap<_, _> = members
        .data
        .into_iter()
        .map(|m| (m.member.id, m.member.nickname))
        .collect();

    let expences =
        std::fs::read_to_string("data/expences.json").context("reading expences file")?;
    let expences: ExpencesList =
        serde_json::from_str(&expences).context("parsing expences json")?;
    let expences: Vec<_> = expences
        .data
        .into_iter()
        .filter(|e| e.expense.status == ExpenceStatus::Active)
        .collect();

    Ok(())
}
