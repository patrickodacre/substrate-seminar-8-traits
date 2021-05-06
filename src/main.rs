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

#[test]
fn test_transfer()
{
    let alice = 1;
    let bob = 2;

    let mut bals = balances::BalancesModule::new();

    bals.set_balance(alice, 10);

    bals.transfer(alice, bob, 5).expect("Transfer Failed");

    assert_eq!(bals.balance(bob), 5);
}

// 2 ways to test for Errors::
#[test]
fn test_transfer_fails_when_from_user_not_exists_1()
{
    let alice = 1;
    let bob = 2;

    let mut bals = balances::BalancesModule::new();

    match bals.transfer(alice, bob, 5) {
        Err(msg) => {
            assert_eq!(msg, "From Account does not exist");
        }
        _ => {
            panic!("Transfer didn't fail as expected");
        }
    }
}

#[test]
fn test_transfer_fails_when_from_user_not_exists_2()
{
    let alice = 1;
    let bob = 2;

    let mut bals = balances::BalancesModule::new();

    let res = bals.transfer(alice, bob, 5).unwrap_err();

    assert_eq!(res, "From Account does not exist");
}
