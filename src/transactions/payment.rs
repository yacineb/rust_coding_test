use crate::dtos::{Transaction, TransactionType};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Default)]
pub struct AccountStatus {
    /// The total funds that are available for trading, staking,withdrawal, etc. Thisshould be equal to the total - held amounts
    available: f64,
    /// The total funds that are held for dispute. This shouldbe equal to total -available amounts
    held: f64,
    /// The total funds that are available or held. This shouldbe equal to available +held
    total: f64,
    /// whether a dispute has occured on the account
    locked: bool,

    #[serde(skip)]
    withdrawals: HashMap<u32, f64>,
}

impl AccountStatus {
    /// Logic of handling transaction according to a given account status
    fn handle_transaction(&mut self, transaction: Transaction) {
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
                self.available += deposit;
                self.total += deposit;
            }
            Transaction {
                r#type: TransactionType::Withdrawal,
                client: _,
                amount: Some(withdraw),
                tx,
            } if withdraw >= 0.0 && withdraw <= self.available => {
                // Execute withdrwal only when client funds are sufficiant
                self.available += withdraw;
                self.total += withdraw;

                self.withdrawals.insert(tx, withdraw);
            }
            Transaction {
                r#type: TransactionType::Dispute,
                client: _,
                amount: _,
                tx,
            } => {
                // handle disputes only for prio non-disputed withdrawals
                if let Some((_, disputed_amount)) = self.withdrawals.remove_entry(&tx) {
                    self.held += disputed_amount;
                    self.available -= disputed_amount;
                }
            }
            _ => (), // ignore any other case
        }
    }
}

pub fn compute_account_statues(transactions: impl Iterator<Item = Transaction>) {
    let mut accounts: HashMap<u16, AccountStatus> = HashMap::new();

    for transaction in transactions {
        println!("processing {:?}", transaction);

        let account = accounts
            .entry(transaction.client)
            .or_insert(Default::default());

        account.handle_transaction(transaction);
    }
}
