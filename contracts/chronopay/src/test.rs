#![cfg(test)]

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{vec, Address, Env, String};

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
fn test_professional_slots_query() {
    let env = setup();
    env.mock_all_auths();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);

    let prof1 = Address::generate(&env);
    let prof2 = Address::generate(&env);

    // Create slots for prof1
    let s1 = client.create_time_slot(&prof1, &1000, &2000);
    let s2 = client.create_time_slot(&prof1, &3000, &4000);

    // Create slot for prof2
    let s3 = client.create_time_slot(&prof2, &5000, &6000);

    // Query prof1 slots
    let slots_prof1 = client.get_professional_slots(&prof1);
    assert_eq!(slots_prof1.len(), 2);
    assert_eq!(slots_prof1.get(0).unwrap(), s1);
    assert_eq!(slots_prof1.get(1).unwrap(), s2);

    // Query prof2 slots
    let slots_prof2 = client.get_professional_slots(&prof2);
    assert_eq!(slots_prof2.len(), 1);
    assert_eq!(slots_prof2.get(0).unwrap(), s3);

    // Query unknown professional
    let unknown_prof = Address::generate(&env);
    let slots_unknown = client.get_professional_slots(&unknown_prof);
    assert_eq!(slots_unknown.len(), 0);
}

#[test]
fn test_create_time_slot_auto_increments() {
    let env = setup();
    env.mock_all_auths();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);

    let prof = Address::generate(&env);

    let slot_id_1 = client.create_time_slot(&prof, &1000u64, &2000u64);
    let slot_id_2 = client.create_time_slot(&prof, &3000u64, &4000u64);
    let slot_id_3 = client.create_time_slot(&prof, &5000u64, &6000u64);

    assert_eq!(slot_id_1, 1);
    assert_eq!(slot_id_2, 2);
    assert_eq!(slot_id_3, 3);
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

#[test]
fn test_mint_and_redeem() {
    let env = setup();
    env.mock_all_auths();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);

    let prof = Address::generate(&env);
    let slot_id = client.create_time_slot(&prof, &1000u64, &2000u64);
    let token = client.mint_time_token(&slot_id);
    assert_eq!(token, Symbol::new(&env, "TIME_TOKEN"));

    let redeemed = client.redeem_time_token(&token);
    assert!(redeemed);
}
