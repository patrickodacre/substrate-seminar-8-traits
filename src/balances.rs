use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

// the Config trait here is a wrapper around required types
// so that we don't have to create a long list of types
// required in our impl of the Balances Module
pub trait Config
{
    type AccountId: Eq + Hash;
    type Balance: Zero + CheckedAdd + CheckedSub + Copy;
}

pub struct BalancesModule<C: Config>
{
    balances: HashMap<C::AccountId, C::Balance>,
}

// this impl is using functions necessary for Hashing - insert, get, etc.
// so we have to use Generic Bounds to constrain this impl to those
// types that have the Traits necessary for those functions:
// https://doc.rust-lang.org/rust-by-example/generics/bounds.html
impl<C: Config> BalancesModule<C>
{
    pub fn new() -> Self
    {
        Self {
            balances: HashMap::new(),
        }
    }

    pub fn balance(&self, who: C::AccountId) -> C::Balance
    {
        *self.balances.get(&who).unwrap_or(&Zero::zero())
    }

    pub fn set_balance(&mut self, who: C::AccountId, amount: C::Balance)
    {
        self.balances.insert(who, amount);
    }

    pub fn transfer(
        &mut self,
        from: C::AccountId,
        to: C::AccountId,
        amount: C::Balance,
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
