#![cfg(test)]

use crate::{
    certificate::PoolPermissions,
    date::BlockDate,
    ledger::{
        check::{CHECK_POOL_REG_MAXIMUM_OPERATORS, CHECK_POOL_REG_MAXIMUM_OWNERS},
        Error,
    },
    testing::{
        builders::{build_stake_pool_registration_cert, StakePoolBuilder, TestTxCertBuilder},
        data::Wallet,
        ConfigBuilder, LedgerBuilder, TestGen,
    },
    value::*,
};
use chain_crypto::{Ed25519, PublicKey};
use std::iter;

#[test]
pub fn pool_registration_is_accepted() {
    let alice = Wallet::from_value(Value(100));
    let bob = Wallet::from_value(Value(100));
    let clarice = Wallet::from_value(Value(100));

    let mut test_ledger = LedgerBuilder::from_config(ConfigBuilder::new())
        .faucets_wallets(vec![&alice, &bob, &clarice])
        .build()
        .expect("cannot build test ledger");

    let stake_pool = StakePoolBuilder::new()
        .with_owners(vec![
            alice.public_key(),
            bob.public_key(),
            clarice.public_key(),
        ])
        .with_pool_permissions(PoolPermissions::new(1))
        .build();

    let certificate = build_stake_pool_registration_cert(&stake_pool.info());
    let fragment = TestTxCertBuilder::new(test_ledger.block0_hash, test_ledger.fee())
        .make_transaction(test_ledger.date(), &[alice, bob, clarice], &certificate);
    assert!(test_ledger
        .apply_fragment(&fragment, test_ledger.date())
        .is_ok());
}

#[test]
pub fn pool_registration_zero_management_threshold() {
    let alice = Wallet::from_value(Value(100));
    let bob = Wallet::from_value(Value(100));
    let clarice = Wallet::from_value(Value(100));

    let mut test_ledger = LedgerBuilder::from_config(ConfigBuilder::new())
        .faucets_wallets(vec![&alice, &bob, &clarice])
        .build()
        .expect("cannot build test ledger");

    let stake_pool = StakePoolBuilder::new()
        .with_owners(vec![
            alice.public_key(),
            bob.public_key(),
            clarice.public_key(),
        ])
        .with_pool_permissions(PoolPermissions::new(0))
        .build();

    let certificate = build_stake_pool_registration_cert(&stake_pool.info());
    let fragment = TestTxCertBuilder::new(test_ledger.block0_hash, test_ledger.fee())
        .make_transaction(test_ledger.date(), &[alice, bob, clarice], &certificate);
    assert_err!(
        Error::PoolRegistrationManagementThresholdZero,
        test_ledger.apply_fragment(&fragment, test_ledger.date())
    );
}

#[test]
pub fn pool_registration_management_threshold_above() {
    let alice = Wallet::from_value(Value(100));
    let bob = Wallet::from_value(Value(100));
    let clarice = Wallet::from_value(Value(100));

    let mut test_ledger = LedgerBuilder::from_config(ConfigBuilder::new())
        .faucets_wallets(vec![&alice, &bob, &clarice])
        .build()
        .expect("cannot build test ledger");

    let stake_pool = StakePoolBuilder::new()
        .with_owners(vec![
            alice.public_key(),
            bob.public_key(),
            clarice.public_key(),
        ])
        .with_pool_permissions(PoolPermissions::new(4))
        .build();

    let certificate = build_stake_pool_registration_cert(&stake_pool.info());
    let fragment = TestTxCertBuilder::new(test_ledger.block0_hash, test_ledger.fee())
        .make_transaction(test_ledger.date(), &[alice, bob, clarice], &certificate);
    assert_err!(
        Error::PoolRegistrationManagementThresholdAbove,
        test_ledger.apply_fragment(&fragment, test_ledger.date())
    );
}

#[test]
pub fn pool_registration_too_many_owners() {
    let alice = Wallet::from_value(Value(100));

    let mut test_ledger = LedgerBuilder::from_config(ConfigBuilder::new())
        .faucets_wallets(vec![&alice])
        .build()
        .expect("cannot build test ledger");

    let owners: Vec<PublicKey<Ed25519>> = iter::from_fn(|| Some(TestGen::public_key()))
        .take(CHECK_POOL_REG_MAXIMUM_OWNERS + 1)
        .collect();

    let stake_pool = StakePoolBuilder::new()
        .with_owners(owners)
        .with_pool_permissions(PoolPermissions::new(4))
        .build();

    let certificate = build_stake_pool_registration_cert(&stake_pool.info());
    let fragment = TestTxCertBuilder::new(test_ledger.block0_hash, test_ledger.fee())
        .make_transaction(test_ledger.date(), &[alice], &certificate);
    assert_err!(
        Error::PoolRegistrationHasTooManyOwners,
        test_ledger.apply_fragment(&fragment, BlockDate::first())
    );
}

#[test]
pub fn pool_registration_too_many_operators() {
    let alice = Wallet::from_value(Value(100));

    let mut test_ledger = LedgerBuilder::from_config(ConfigBuilder::new())
        .faucets_wallets(vec![&alice])
        .build()
        .expect("cannot build test ledger");

    let operators: Vec<PublicKey<Ed25519>> = iter::from_fn(|| Some(TestGen::public_key()))
        .take(CHECK_POOL_REG_MAXIMUM_OPERATORS + 1)
        .collect();

    let stake_pool = StakePoolBuilder::new()
        .with_owners(vec![alice.public_key()])
        .with_operators(operators)
        .with_pool_permissions(PoolPermissions::new(1))
        .build();

    let certificate = build_stake_pool_registration_cert(&stake_pool.info());
    let fragment = TestTxCertBuilder::new(test_ledger.block0_hash, test_ledger.fee())
        .make_transaction(test_ledger.date(), &[alice], &certificate);
    assert_err!(
        Error::PoolRegistrationHasTooManyOperators,
        test_ledger.apply_fragment(&fragment, BlockDate::first())
    );
}

#[test]
#[should_panic]
pub fn pool_registration_zero_signatures() {
    let alice = Wallet::from_value(Value(100));

    let mut test_ledger = LedgerBuilder::from_config(ConfigBuilder::new())
        .faucets_wallets(vec![&alice])
        .build()
        .expect("cannot build test ledger");

    let stake_pool = StakePoolBuilder::new()
        .with_owners(vec![alice.public_key()])
        .with_pool_permissions(PoolPermissions::new(1))
        .build();

    let certificate = build_stake_pool_registration_cert(&stake_pool.info());
    let fragment = TestTxCertBuilder::new(test_ledger.block0_hash, test_ledger.fee())
        .make_transaction_different_signers(test_ledger.date(), &alice, &[], &certificate);
    test_ledger
        .apply_fragment(&fragment, test_ledger.date())
        .unwrap();
}

#[test]
pub fn pool_registration_too_many_signatures() {
    let alice = Wallet::from_value(Value(100));

    let mut test_ledger = LedgerBuilder::from_config(ConfigBuilder::new())
        .faucets_wallets(vec![&alice])
        .build()
        .expect("cannot build test ledger");

    let signers: Vec<Wallet> = iter::from_fn(|| Some(Wallet::from_value(Value(1000))))
        .take(CHECK_POOL_REG_MAXIMUM_OWNERS + 1)
        .collect();

    let stake_pool = StakePoolBuilder::new()
        .with_owners(vec![alice.public_key()])
        .with_pool_permissions(PoolPermissions::new(1))
        .build();

    let certificate = build_stake_pool_registration_cert(&stake_pool.info());
    let fragment = TestTxCertBuilder::new(test_ledger.block0_hash, test_ledger.fee())
        .make_transaction_different_signers(test_ledger.date(), &alice, &signers, &certificate);
    assert_err!(
        Error::CertificateInvalidSignature,
        test_ledger.apply_fragment(&fragment, test_ledger.date())
    );
}
