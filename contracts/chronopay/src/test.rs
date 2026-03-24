#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, String};

#[test]
fn test_hello() {
    let env = Env::default();
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
fn test_create_time_slot_auto_increments() {
    let env = Env::default();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);

    let slot_id_1 = client.try_create_time_slot(
        &String::from_str(&env, "professional_alice"),
        &1000u64,
        &2000u64,
    );
    let slot_id_2 = client.try_create_time_slot(
        &String::from_str(&env, "professional_alice"),
        &3000u64,
        &4000u64,
    );
    let slot_id_3 = client.try_create_time_slot(
        &String::from_str(&env, "professional_alice"),
        &5000u64,
        &6000u64,
    );

    assert_eq!(slot_id_1, Ok(Ok(1)));
    assert_eq!(slot_id_2, Ok(Ok(2)));
    assert_eq!(slot_id_3, Ok(Ok(3)));
}

#[test]
fn test_mint_and_redeem() {
    let env = Env::default();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);

    let slot_id = client.try_create_time_slot(&String::from_str(&env, "pro"), &1000u64, &2000u64);
    assert_eq!(slot_id, Ok(Ok(1)));
    let token = client.mint_time_token(&1u32);
    assert_eq!(token, soroban_sdk::Symbol::new(&env, "TIME_TOKEN"));

    let redeemed = client.redeem_time_token(&token);
    assert!(redeemed);
}

#[test]
fn test_create_time_slot_end_time_equals_start_time_fails() {
    let env = Env::default();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);

    let result = client.try_create_time_slot(
        &String::from_str(&env, "professional_bob"),
        &1000u64,
        &1000u64,
    );
    assert_eq!(result, Err(Ok(ContractError::EndTimeBeforeStartTime)));
}

#[test]
fn test_create_time_slot_end_time_before_start_time_fails() {
    let env = Env::default();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);

    let result = client.try_create_time_slot(
        &String::from_str(&env, "professional_carol"),
        &2000u64,
        &1000u64,
    );
    assert_eq!(result, Err(Ok(ContractError::EndTimeBeforeStartTime)));
}

#[test]
fn test_create_time_slot_valid_adjacent_times() {
    let env = Env::default();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);

    let result = client.try_create_time_slot(
        &String::from_str(&env, "professional_dave"),
        &1000u64,
        &1001u64,
    );
    assert_eq!(result, Ok(Ok(1)));
}

#[test]
fn test_create_time_slot_valid_large_time_difference() {
    let env = Env::default();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);

    let result = client.try_create_time_slot(
        &String::from_str(&env, "professional_eve"),
        &1000000000u64,
        &2000000000u64,
    );
    assert_eq!(result, Ok(Ok(1)));
}

#[test]
fn test_create_time_slot_stores_correct_data() {
    let env = Env::default();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);

    let result = client.try_create_time_slot(
        &String::from_str(&env, "professional_frank"),
        &5000u64,
        &8000u64,
    );
    assert_eq!(result, Ok(Ok(1)));

    let stored_slot: TimeSlot = env.as_contract(&contract_id, || {
        env.storage()
            .instance()
            .get(&DataKey::TimeSlot(1))
            .expect("time slot not found")
    });

    assert_eq!(
        stored_slot.professional,
        String::from_str(&env, "professional_frank")
    );
    assert_eq!(stored_slot.start_time, 5000u64);
    assert_eq!(stored_slot.end_time, 8000u64);
}

#[test]
fn test_create_time_slot_multiple_valid_slots() {
    let env = Env::default();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);

    let slot_id_1 = client.try_create_time_slot(&String::from_str(&env, "pro_1"), &100u64, &200u64);
    let slot_id_2 = client.try_create_time_slot(&String::from_str(&env, "pro_2"), &300u64, &400u64);

    assert_eq!(slot_id_1, Ok(Ok(1)));
    assert_eq!(slot_id_2, Ok(Ok(2)));

    let slot_1: TimeSlot = env.as_contract(&contract_id, || {
        env.storage().instance().get(&DataKey::TimeSlot(1)).unwrap()
    });
    let slot_2: TimeSlot = env.as_contract(&contract_id, || {
        env.storage().instance().get(&DataKey::TimeSlot(2)).unwrap()
    });

    assert_eq!(slot_1.start_time, 100u64);
    assert_eq!(slot_1.end_time, 200u64);
    assert_eq!(slot_2.start_time, 300u64);
    assert_eq!(slot_2.end_time, 400u64);
}
