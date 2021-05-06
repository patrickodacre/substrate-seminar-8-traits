use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

pub struct BalancesModule<AccountId, Balance>
{
    balances: HashMap<AccountId, Balance>,
}

impl<AccountId, Balance> BalancesModule<AccountId, Balance>
where
    AccountId: Eq + Hash,
    Balance: CheckedAdd + CheckedSub + Copy + Zero,
{
    pub fn new() -> Self
    {
        Self {
            balances: HashMap::new(),
        }
    }

    pub fn balance(&self, who: AccountId) -> Balance
    {
        *self.balances.get(&who).unwrap_or(&Zero::zero())
    }

    pub fn set_balance(&mut self, who: AccountId, amount: Balance)
    {
        self.balances.insert(who, amount);
    }

    pub fn transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<(), &'static str>
    {
        let from_balance = self
            .balances
            .get(&from)
            .ok_or("From Account does not exist")?;

        let zero = Zero::zero();
        let to_balance = self.balances.get(&to).unwrap_or(&zero);

        let new_from_balance = from_balance
            .checked_sub(&amount)
            .ok_or("Insufficient Funds")?;

        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

        self.balances.insert(from, new_from_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}
