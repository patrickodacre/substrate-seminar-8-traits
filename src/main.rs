mod balances;

fn main() {}

#[test]
fn test_set_balance()
{
    let alice = 1;

    let mut bals = balances::BalancesModule::new();
    assert_eq!(bals.balance(alice), 0);

    bals.set_balance(alice, 10);
    assert_eq!(bals.balance(alice), 10);
}
