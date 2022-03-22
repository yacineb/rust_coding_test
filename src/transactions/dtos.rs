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
    pub r#type: TransactionType,

    /// Client Identifier
    pub client: u16,

    /// Transaction Identifier
    pub tx: u32,

    /// Transaction amount
    pub amount: Option<f64>,
}
