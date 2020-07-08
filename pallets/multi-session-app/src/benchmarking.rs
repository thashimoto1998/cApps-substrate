//! Multi session app benchmarking

#![cfg(feature = "runtime-benchmarks")]
use super::*;
use frame_system::{RawOrigin, Module as System};
use frame_benchmarking::{benchmarks, account};
use crate::Module as MultiApp;

const SEED: u32 = 0;

fn get_state_proof<T: Trait>(
   seq: u128,
   state: u8,
   timeout: T::BlockNumber,
   session_id: T::Hash 
) -> StateProof<T::BlockNumber, T::Hash> {
    let app_state = AppState {
        seq_num: seq,
        state: state,
        timeout: timeout,
        session_id: session_id
    };

    let state_proof = StateProof {
        app_state: app_state,
    };

    return state_proof;
}

benchmarks! {
    _{ }

    app_initiate {
        let i in 0 .. 1000;
        let mut player1 = T::AccountId::default();
        let mut player2 = T::AccountId::default();
        player1 = account("player1", i, SEED);
        player2 = account("player2", i, SEED);
        let mut player = vec![];
        if player1 < player2 {
            player.push(player1.clone());
            player.push(player2.clone());
        } else {
            player.push(player2.clone());
            player.push(player1.clone());
        }

        let initiate_request = SessionInitiateRequest {
            nonce: i as u128,
            player_num: 2,
            players: players.clone(),
            timeout: 2.into()
        };
    }: _(RawOrigin::Signed(player1.clone()), initiate_request)

    update_by_state {
        let i in 0 .. 1000;
        let mut player1 = T::AccountId::default();
        let mut player2 = T::AccountId::default();
        player1 = account("player1", i, SEED);
        player2 = account("player2", i, SEED);
        let mut player = vec![];
        if player1 < player2 {
            player.push(player1.clone());
            player.push(player2.clone());
        } else {
            player.push(player2.clone());
            player.push(player1.clone());
        }

        let initiate_request = SessionInitiateRequest {
            nonce: i as u128,
            player_num: 2,
            players: players.clone(),
            timeout: 2.into()
        };
        MultiApp::<T>::app_initiate(RawOrigin::Signed(player1.clone()).into(), initiate_request.clone())?;
        let session_id = MultiApp::<T>::get_app_id(initiate_request.nonce, initiate_request.players.clone());

        let state_proof = get_state_proof::<T>(i as u128, 5, 2.into(), session_id);
    }: _(RawOrigin::Signed(player1.clone()), state_proof)

    update_by_action {
        let i in 0 .. 1000;
        let mut player1 = T::AccountId::default();
        let mut player2 = T::AccountId::default();
        player1 = account("player1", i, SEED);
        player2 = account("player2", i, SEED);
        let mut player = vec![];
        if player1 < player2 {
            player.push(player1.clone());
            player.push(player2.clone());
        } else {
            player.push(player2.clone());
            player.push(player1.clone());
        }

        let initiate_request = SessionInitiateRequest {
            nonce: i as u128,
            player_num: 2,
            players: players.clone(),
            timeout: 2.into()
        };
        MultiApp::<T>::app_initiate(RawOrigin::Signed(player1.clone()), initiate_request.clone())?;
        let session_id = MultiApp::<T>::get_app_id(initiate_request.nonce, initiate_request.players.clone());
        let state_proof = get_state_proof::<T>(i as u128, 3, 2.into(), session_id);
        MultiApp::<T>::update_by_state(RawOrigin::Signed(player1.clone()).into(), state_proof)?;
        let settle_finalized_time = MultiApp::<T>::get_settle_finalized_time(session_id).unwrap();
        System::<T>::set_block_number(settle_finalized_time + 1.into());
    }: _(RawOrigin::Signed(player1.clone()), session_id, 3)

    finalize_on_action_timeout {
        let i in 0 .. 1000;
        let mut player1 = T::AccountId::default();
        let mut player2 = T::AccountId::default();
        player1 = account("player1", i, SEED);
        player2 = account("player2", i, SEED);
        let mut player = vec![];
        if player1 < player2 {
            player.push(player1.clone());
            player.push(player2.clone());
        } else {
            player.push(player2.clone());
            player.push(player1.clone());
        }

        let initiate_request = SessionInitiateRequest {
            nonce: i as u128,
            player_num: 2,
            players: players.clone(),
            timeout: 2.into()
        };
        MultiApp::<T>::app_initiate(RawOrigin::Signed(player1.clone()), initiate_request.clone())?;
        let session_id = MultiApp::<T>::get_app_id(initiate_request.nonce, initiate_request.players.clone());
        let state_proof = get_state_proof::<T>(i as u128, 2, 2.into(), session_id);
        MultiApp::<T>::update_by_state(RawOrigin::Signed(player1.clone()).into(), state_proof)?;

        // advance block number after action timeout
        System::<T>::set_block_number(5.into());
    }: _(RawOrigin::Signed(player1.clone()), session_id)

    is_finalized {
        let i in 0 .. 1000;
        let mut player1 = T::AccountId::default();
        let mut player2 = T::AccountId::default();
        player1 = account("player1", i, SEED);
        player2 = account("player2", i, SEED);
        let mut player = vec![];
        if player1 < player2 {
            player.push(player1.clone());
            player.push(player2.clone());
        } else {
            player.push(player2.clone());
            player.push(player1.clone());
        }

        let initiate_request = SessionInitiateRequest {
            nonce: i as u128,
            player_num: 2,
            players: players.clone(),
            timeout: 2.into()
        };
        MultiApp::<T>::app_initiate(RawOrigin::Signed(player1.clone()), initiate_request.clone())?;
        let session_id = MultiApp::<T>::get_app_id(initiate_request.nonce, initiate_request.players.clone());
        let state_proof = get_state_proof::<T>(i as u128, 2, 2.into(), session_id);
        MultiApp::<T>::update_by_state(RawOrigin::Signed(player1.clone()).into(), state_proof)?;
    }: _(RawOrigin::Signed(player1.clone()), session_id)

    get_outcome {
        let i in 0 .. 1000;
        let mut player1 = T::AccountId::default();
        let mut player2 = T::AccountId::default();
        player1 = account("player1", i, SEED);
        player2 = account("player2", i, SEED);
        let mut player = vec![];
        if player1 < player2 {
            player.push(player1.clone());
            player.push(player2.clone());
        } else {
            player.push(player2.clone());
            player.push(player1.clone());
        }

        let initiate_request = SessionInitiateRequest {
            nonce: i as u128,
            player_num: 2,
            players: players.clone(),
            timeout: 2.into()
        };
        MultiApp::<T>::app_initiate(RawOrigin::Signed(player1.clone()), initiate_request.clone())?;
        let session_id = MultiApp::<T>::get_app_id(initiate_request.nonce, initiate_request.players.clone());
        let state_proof = get_state_proof::<T>(i as u128, 2, 2.into(), session_id);
        MultiApp::<T>::update_by_state(RawOrigin::Signed(player1.clone()).into(), state_proof)?;
    }: _(RawOrigin::Signed(player1.clone()), session_id, 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::*;
    use frame_support::assert_ok;

    #[test]
    fn app_initiate() {
        ExtBuilder::build().execute_with(|| {
            assert_ok!(test_benchmark_app_initiate::<Test>());
        })
    }

    #[test]
    fn update_by_state() {
        ExtBuilder::build().execute_with(|| {
            assert_ok!(test_benchmark_update_by_state::<Test>());
        })
    }

    #[test]
    fn update_by_action() {
        ExtBuilder::build().execute_with(|| {
            assert_ok!(test_benchmark_update_by_action::<Test>());
        })
    }

    #[test]
    fn finalize_on_action_timeout() {
        ExtBuilder::build().execute_with(|| {
            assert_ok!(test_benchmark_finalize_on_action_timeout::<Test>());
        })
    }

    #[test]
    fn is_finalized() {
        ExtBuilder::build().execute_with(|| {
            assert_ok!(test_benchmark_is_finalized::<Test>());
        })
    }

    #[test]
    fn get_outcome() {
        ExtBuilder::build().execute_with(|| {
            assert_ok!(test_benchmark_get_outcome::<Test>());
        })
    }
}