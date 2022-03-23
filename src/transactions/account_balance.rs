use serde::Serialize;
use std::collections::HashMap;

fn round_4decimals_serialize<S>(x: &f64, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_f64((x * 10000.0).round() / 10000.0)
}

#[derive(Debug, Serialize, Default)]
pub struct AccountBalance {
    /// Client Identifier
    pub client: u16,

    /// The total funds that are available for trading, staking,withdrawal, etc. Thisshould be equal to the total - held amounts
    #[serde(serialize_with = "round_4decimals_serialize")]
    pub available: f64,
    /// The total funds that are held for dispute. This shouldbe equal to total -available amounts
    #[serde(serialize_with = "round_4decimals_serialize")]
    pub held: f64,
    /// The total funds that are available or held. This shouldbe equal to available +held
    #[serde(serialize_with = "round_4decimals_serialize")]
    pub total: f64,
    /// whether a dispute has occured on the account
    pub locked: bool,

    #[serde(skip)]
    pub withdrawals: HashMap<u32, f64>,

    #[serde(skip)]
    pub disputes: HashMap<u32, f64>,
}
