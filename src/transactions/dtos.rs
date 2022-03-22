use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    r#type: TransactionType,

    /// Client Identifier
    client: u16,

    /// Transaction Identifier
    tx: u32,

    /// Transaction amount
    amount: Option<f64>,
}
