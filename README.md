# Testing

Some input sample testing csv data are in folder sample_data.

expected outputs have out.csv extension

- 1.csv: basic test case in the coding test
- 2.csv: in this scenario , a chargeback on a withdraw that never happened (insufficiant funds) is ignored
- 3.csv: ignore disputes, resolves,chargebacks on non-existing transactions
- 4.csv: a scenario of a chargback
- 5.csv: a scenario of a resolved dispute
- 6.csv : same than previous but with a chargeback on a transaction on which is not under dispute
