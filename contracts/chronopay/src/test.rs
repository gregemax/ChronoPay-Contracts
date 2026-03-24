#![cfg(test)]

use super::*;
use crate::events::{SlotCreated, TokenMinted, TokenPurchased, TokenRedeemed};
use soroban_sdk::events::Event;
use soroban_sdk::testutils::Events as _;
use soroban_sdk::{vec, Env, String, Symbol};

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

    let slot_id_1 = client.create_time_slot(
        &String::from_str(&env, "professional_alice"),
        &1000u64,
        &2000u64,
    );
    let slot_id_2 = client.create_time_slot(
        &String::from_str(&env, "professional_alice"),
        &3000u64,
        &4000u64,
    );
    let slot_id_3 = client.create_time_slot(
        &String::from_str(&env, "professional_alice"),
        &5000u64,
        &6000u64,
    );

    assert_eq!(slot_id_1, 1);
    assert_eq!(slot_id_2, 2);
    assert_eq!(slot_id_3, 3);
}

#[test]
fn test_mint_and_redeem() {
    let env = Env::default();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);

    let slot_id = client.create_time_slot(&String::from_str(&env, "pro"), &1000u64, &2000u64);
    let token = client.mint_time_token(&slot_id);
    assert_eq!(token, soroban_sdk::Symbol::new(&env, "TIME_TOKEN"));

    let redeemed = client.redeem_time_token(&token);
    assert!(redeemed);
}

#[test]
fn test_create_time_slot_emits_event() {
    let env = Env::default();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);

    let professional = String::from_str(&env, "alice");
    client.create_time_slot(&professional, &1000u64, &2000u64);

    let addr = contract_id.clone();
    let expected = SlotCreated {
        professional,
        slot_id: 1,
        start_time: 1000,
        end_time: 2000,
    };
    let all = env.events().all();
    let filtered = all.filter_by_contract(&addr);
    assert_eq!(filtered, [expected.to_xdr(&env, &addr)]);
}

#[test]
fn test_mint_time_token_emits_event() {
    let env = Env::default();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);

    let slot_id = client.create_time_slot(&String::from_str(&env, "pro"), &100u64, &200u64);
    let token_id = client.mint_time_token(&slot_id);

    let addr = contract_id.clone();
    let expected_mint = TokenMinted { slot_id, token_id };
    let all = env.events().all();
    let filtered = all.filter_by_contract(&addr);
    assert_eq!(filtered, [expected_mint.to_xdr(&env, &addr)]);
}

#[test]
fn test_buy_time_token_emits_event() {
    let env = Env::default();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);

    let token_id = Symbol::new(&env, "TIME_TOKEN");
    let buyer = String::from_str(&env, "buyer_bob");
    let seller = String::from_str(&env, "seller_carol");

    client.buy_time_token(&token_id, &buyer, &seller);

    let addr = contract_id.clone();
    let expected = TokenPurchased {
        token_id,
        buyer,
        seller,
    };
    let all = env.events().all();
    let filtered = all.filter_by_contract(&addr);
    assert_eq!(filtered, [expected.to_xdr(&env, &addr)]);
}

#[test]
fn test_redeem_time_token_emits_event() {
    let env = Env::default();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);

    let token_id = Symbol::new(&env, "TIME_TOKEN");
    client.redeem_time_token(&token_id);

    let addr = contract_id.clone();
    let expected = TokenRedeemed { token_id };
    let all = env.events().all();
    let filtered = all.filter_by_contract(&addr);
    assert_eq!(filtered, [expected.to_xdr(&env, &addr)]);
}

#[test]
fn test_multiple_operations_emit_ordered_events() {
    let env = Env::default();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);

    let addr = contract_id.clone();

    // Each invocation's events are verified independently (all() returns last invocation only)
    let professional = String::from_str(&env, "pro");
    let slot_id = client.create_time_slot(&professional, &100u64, &200u64);
    let filtered = env.events().all().filter_by_contract(&addr);
    assert_eq!(
        filtered,
        [SlotCreated {
            professional,
            slot_id: 1,
            start_time: 100,
            end_time: 200,
        }
        .to_xdr(&env, &addr)]
    );

    let token_id = client.mint_time_token(&slot_id);
    let filtered = env.events().all().filter_by_contract(&addr);
    assert_eq!(
        filtered,
        [TokenMinted {
            slot_id,
            token_id: token_id.clone(),
        }
        .to_xdr(&env, &addr)]
    );

    let buyer = String::from_str(&env, "buyer");
    let seller = String::from_str(&env, "seller");
    client.buy_time_token(&token_id, &buyer, &seller);
    let filtered = env.events().all().filter_by_contract(&addr);
    assert_eq!(
        filtered,
        [TokenPurchased {
            token_id: token_id.clone(),
            buyer,
            seller,
        }
        .to_xdr(&env, &addr)]
    );

    client.redeem_time_token(&token_id);
    let filtered = env.events().all().filter_by_contract(&addr);
    assert_eq!(filtered, [TokenRedeemed { token_id }.to_xdr(&env, &addr)]);
}
