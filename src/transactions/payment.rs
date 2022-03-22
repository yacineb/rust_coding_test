use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AccountStatus {
    /// Client Identifier
    client: u16,
    /// The total funds that are available for trading, staking,withdrawal, etc. Thisshould be equal to the total - held amounts
    available: f64,
    /// The total funds that are held for dispute. This shouldbe equal to total -available amounts
    held: f64,
    /// The total funds that are available or held. This shouldbe equal to available +held
    total: f64,
    /// whether a dispute has occured on the account
    locked: bool,
}
