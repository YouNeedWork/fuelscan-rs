use models::{
    account::{Account, AccountType},
    call::{Call, CallType},
};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

pub fn process_account(calls: &Vec<Call>) -> Vec<Account> {
    let mut accounts = Vec::new();

    //TODO simple solution, if there have a contract call, maybe will create ohther account.
    for call in calls {
        let mut caller = Account::default();
        caller.account_hash = call.sender.clone();
        caller.account_type = AccountType::Account;
        caller.gas_used = call.gas_used;
        caller.sender_count = 1;
        accounts.push(caller);
        if call.call_type == CallType::Contract {
            let mut receiver = Account::default();
            receiver.account_hash = call.receiver.clone();
            receiver.account_type = AccountType::Contract;
            receiver.gas_used = call.gas_used;
            receiver.recever_count = 1;
            accounts.push(receiver);
        } else {
            let mut receiver = Account::default();
            receiver.account_hash = call.receiver.clone();
            receiver.account_type = AccountType::Account;
            receiver.gas_used = call.gas_used;
            receiver.recever_count = 1;
            accounts.push(receiver);
        }
    }
    // fillter save hash and put all count in to one
    let mut account_map = std::collections::HashMap::new();

    for account in accounts {
        let account_hash = account.account_hash.clone();
        if account_map.contains_key(&account_hash) {
            let acc: &mut Account = account_map.get_mut(&account_hash).unwrap();
            acc.gas_used += account.gas_used;
            acc.sender_count += account.sender_count;
            acc.recever_count += account.recever_count;
        } else {
            account_map.insert(account_hash, account);
        }
    }

    account_map
        .into_par_iter()
        .map(|(_, v)| v)
        .collect::<Vec<_>>()
}
