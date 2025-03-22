use ic_ledger_types::{AccountIdentifier, AccountBalanceArgs, MAINNET_LEDGER_CANISTER_ID, account_balance};

/// Lookup the balance of an ICP account.
pub async fn lookup_account(account: &str) -> String {
    if account.len() != 64 {
        return "Account must be 64 characters long".to_string();
    }

    match AccountIdentifier::from_hex(account) {
        Ok(account) => {
            let balance = account_balance(
                MAINNET_LEDGER_CANISTER_ID,
                AccountBalanceArgs {
                    account,
                }
            ).await.expect("call to ledger failed");

            format!("Balance of {} is {} ICP", account, balance)
        }
        Err(_) => "Invalid account".to_string(),
    }
}