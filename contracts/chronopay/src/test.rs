#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Address, Env, String};
use soroban_sdk::testutils::Address as _;

fn setup() -> Env {
    Env::default()
}

#[test]
fn test_hello() {
    let env = setup();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);

    let words = client.hello(&String::from_str(&env, "Dev"));
    assert_eq!(
        words,
        vec![
            &env,
            String::from_str(&env, "ChronoPay"),
            String::from_str(&env, "Dev"),
        ]
    );
}

#[test]
fn test_create_and_query_slot() {
    let env = setup();
    env.mock_all_auths();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);

    let professional = Address::generate(&env);
    let start_time = 1000u64;
    let end_time = 2000u64;

    let slot_id = client.create_time_slot(&professional, &start_time, &end_time);
    assert_eq!(slot_id, 1);

    // Query existing slot
    let slot = client.get_time_slot(&slot_id).expect("slot should exist");
    assert_eq!(slot.professional, professional);
    assert_eq!(slot.start_time, start_time);
    assert_eq!(slot.end_time, end_time);
    assert!(slot.token.is_none());

    // Query non-existent slot
    let non_existent = client.get_time_slot(&999u32);
    assert!(non_existent.is_none());
}

#[test]
#[should_panic(expected = "end_time must be after start_time")]
fn test_create_time_slot_rejects_invalid_times() {
    let env = setup();
    env.mock_all_auths();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);
    let professional = Address::generate(&env);
    let _ = client.create_time_slot(&professional, &10u64, &10u64);
}
