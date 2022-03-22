use introspection::*;
use introspection_derive::*;

use crate::dtos::{Transaction, TransactionType};
use serde::Deserialize;
use std::collections::HashMap;

#[cfg(feature = "dead_code")]
fn round_4decimals_serialize<S>(x: &f64, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_f64((x * 10000.0).round() / 10000.0)
}

#[derive(Debug, Deserialize, Default, Introspection)]
pub struct AccountStatus {
    /// Client Identifier
    client: u16,

    /// The total funds that are available for trading, staking,withdrawal, etc. Thisshould be equal to the total - held amounts
    #[serde(serialize_with = "round_4decimals_serialize")]
    available: f64,
    /// The total funds that are held for dispute. This shouldbe equal to total -available amounts
    #[serde(serialize_with = "round_4decimals_serialize")]
    held: f64,
    /// The total funds that are available or held. This shouldbe equal to available +held
    #[serde(serialize_with = "round_4decimals_serialize")]
    total: f64,
    /// whether a dispute has occured on the account
    locked: bool,

    #[serde(skip)]
    withdrawals: HashMap<u32, f64>,

    #[serde(skip)]
    disputes: HashMap<u32, f64>,
}

impl AccountStatus {
    /// Logic of handling transaction according to a given account status
    fn handle_transaction(&mut self, transaction: Transaction) {
        assert_eq!(transaction.client, self.client);
        // case when client account has been frozen due to a chargeback
        if self.locked {
            return;
        }

        match transaction {
            Transaction {
                r#type: TransactionType::Deposit,
                client: _,
                amount: Some(deposit),
                tx: _,
            } if deposit >= 0.0 => {
                // add deposit amount
                self.available += deposit;
                self.total += deposit;
            }
            Transaction {
                r#type: TransactionType::Withdrawal,
                client: _,
                amount: Some(withdraw),
                tx,
            } if withdraw >= 0.0 && withdraw <= self.available => {
                // Execute withdrawal only when client funds are sufficiant
                self.available -= withdraw;
                self.total -= withdraw;

                self.withdrawals.insert(tx, withdraw);
            }
            Transaction {
                r#type: TransactionType::Dispute,
                client: _,
                amount: _,
                tx,
            } => {
                // handle disputes only for prio non-disputed withdrawals
                // case of disputes for deposit operations, was not handled. i'm not sure if this makes sense in a bank business..
                if let Some((_, disputed_amount)) = self.withdrawals.remove_entry(&tx) {
                    self.held += disputed_amount;
                    self.available -= disputed_amount;

                    // add to client disputes
                    self.disputes.insert(tx, disputed_amount);
                }
            }
            Transaction {
                r#type: TransactionType::Resolve,
                client: _,
                amount: _,
                tx,
            } => {
                // lookup for an existing dispute, and handle its resolution (reverse of dispute operation)
                if let Some(disputed_amount) = self.disputes.get(&tx) {
                    self.held -= disputed_amount;
                    self.available += disputed_amount;
                }
            }
            Transaction {
                r#type: TransactionType::Chargeback,
                client: _,
                amount: _,
                tx,
            } => {
                // lookup for an existing dispute, and handle its resolution
                if let Some((_, disputed_amount)) = self.disputes.remove_entry(&tx) {
                    self.held -= disputed_amount;
                    self.total += disputed_amount;

                    // freeze the account immediately after a chargeback
                    self.locked = true;
                }
            }
            _ => (), // ignore any other case
        }
    }
}

pub fn compute_account_statues(
    transactions: impl Iterator<Item = Transaction>,
) -> impl Iterator<Item = AccountStatus> {
    let mut accounts: HashMap<u16, AccountStatus> = HashMap::new();

    for transaction in transactions {
        println!("processing {:?}", transaction);

        let client_id = transaction.client;
        let account = accounts.entry(client_id).or_insert(Default::default());
        account.client = client_id;
        account.handle_transaction(transaction);
    }

    // ordering by client id does not matter
    accounts.into_iter().map(|(_, x)| x)
}
