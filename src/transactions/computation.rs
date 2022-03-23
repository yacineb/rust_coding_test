use crate::transactions::account_balance::AccountBalance;
use crate::transactions::transaction::*;
use std::collections::HashMap;

impl AccountBalance {
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

/// Computes account balances for a set of Transactions
pub fn compute(
    transactions: impl Iterator<Item = Transaction>,
) -> impl Iterator<Item = AccountBalance> {
    let mut accounts: HashMap<u16, AccountBalance> = HashMap::new();

    for transaction in transactions {
        let client_id = transaction.client;
        let account = accounts.entry(client_id).or_insert(Default::default());
        account.client = client_id;
        account.handle_transaction(transaction);
    }

    // ordering by client id does not matter
    accounts.into_iter().map(|(_, x)| x)
}
