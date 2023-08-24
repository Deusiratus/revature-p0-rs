use crate::account::Account;

struct Transaction {
    id: usize,
    transaction_type: TransactionType,
}

enum TransactionType {
    Withdrawl(f64),
    Deposit(f64),
    Transfer {amount: f64, target: Account },
}