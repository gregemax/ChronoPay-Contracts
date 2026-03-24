#![no_std]
//! ChronoPay time token contract — stub for create_time_slot, mint_time_token, buy_time_token, redeem_time_token.

use soroban_sdk::{contract, contractimpl, contracttype, vec, Address, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimeTokenStatus {
    Available,
    Sold,
    Redeemed,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimeSlot {
    pub professional: Address,
    pub start_time: u64,
    pub end_time: u64,
    pub minted: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    SlotSeq,
    Slot(u32),
    Owner,
    Status,
}

#[contract]
pub struct ChronoPayContract;

#[contractimpl]
impl ChronoPayContract {
    /// Create a time slot with an auto-incrementing slot id.
    /// Requires authorization from the professional.
    /// Returns the newly assigned slot id.
    pub fn create_time_slot(
        env: Env,
        professional: Address,
        start_time: u64,
        end_time: u64,
    ) -> u32 {
        professional.require_auth();

        let current_seq: u32 = env
            .storage()
            .instance()
            .get(&DataKey::SlotSeq)
            .unwrap_or(0u32);

        let next_seq = current_seq.checked_add(1).expect("slot id overflow");

        env.storage().instance().set(&DataKey::SlotSeq, &next_seq);

        let slot = TimeSlot {
            professional,
            start_time,
            end_time,
            minted: false,
        };
        env.storage()
            .instance()
            .set(&DataKey::Slot(next_seq), &slot);

        next_seq
    }

    /// Mint a time token for a slot (stub).
    /// Requires authorization from the professional who created the slot.
    pub fn mint_time_token(env: Env, slot_id: u32) -> Symbol {
        let slot_key = DataKey::Slot(slot_id);
        let mut slot: TimeSlot = env
            .storage()
            .instance()
            .get(&slot_key)
            .expect("slot not found");

        if slot.minted {
            panic!("token already minted for this slot");
        }

        // Ensure the professional who created the slot authorizes minting
        slot.professional.require_auth();

        slot.minted = true;
        env.storage().instance().set(&slot_key, &slot);

        Symbol::new(&env, "TIME_TOKEN")
    }

    /// Buy / transfer time token (stub). In full implementation: token_id, buyer, seller, price.
    pub fn buy_time_token(env: Env, token_id: Symbol, buyer: Address, seller: Address) -> bool {
        let _ = (token_id, buyer, seller);
        env.storage()
            .instance()
            .set(&DataKey::Owner, &env.current_contract_address());
        true
    }

    /// Redeem time token (stub). In full implementation: token_id, marks as redeemed.
    pub fn redeem_time_token(env: Env, token_id: Symbol) -> bool {
        let _ = token_id;
        env.storage()
            .instance()
            .set(&DataKey::Status, &TimeTokenStatus::Redeemed);
        true
    }

    /// Hello-style entrypoint for CI and SDK sanity check.
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "ChronoPay"), to]
    }
}

mod test;
