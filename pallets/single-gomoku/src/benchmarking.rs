//! Single gomoku benchmarking

#![cfg(feature = "runtime-benchmarks")]
use super::*;
use frame_system::{RawOrigin, Module as System};
use frame_benchmarking::{benchmarks, account};
use crate::Module as SingleGomoku;
use std::vec::Vec;

const SEED: u32 = 0;

fn get_state_proof<T: Trait>(
    nonce: u128, 
    seq: u128, 
    board_state: Vec<u8>,
    timeout: T::BlockNumber, 
    app_id: T::Hash
) -> StateProof<T::BlockNumber, T::Hash> {
    let app_state = AppState {
        nonce: nonce,
        seq_num: seq,
        board_state: board_state,
        timeout: timeout,
        app_id: app_id,
    };
    let state_proof = StateProof {
        app_state: app_state,
    };

    return state_proof;
}

fn place_stone(
    app_id: T::Hash,
    nonce: u128,
    players: Vec<T::AccountId>,
) {
    let mut board_state_1 = vec![0; 227];
    board_state_1[0] = 0;
    board_state_1[1] = 1;
    board_state_1[2] = 2;
    board_state_1[3] = 2;
    board_state_1[4] = 1;
    board_state_1[5] = 1;
    board_state_1[6] = 2;
    board_state_1[7] = 2;
    board_state_1[8] = 1;
    let state_proof_1 = get_state_proof(nonce, 1, board_state_1.clone(), 0.into(), app_id);
    SingleGomoku::<T>::update_by_state(RawOrigin::Signed(palyers[0].clone()).into(), state_proof_1);

    let mut board_state_2 = vec![0; 227];
    board_state_2[0] = 0; // winner
    board_state_2[1] = 2; // turn
    board_state_2[2] = 1; // (0, 0)
    board_state_2[3] = 1; // (0, 1)
    board_state_2[4] = 1; // (0, 2)
    board_state_2[5] = 1; // (0, 3)
    board_state_2[101] = 2; 
    board_state_2[102] = 2;
    board_state_2[103] = 2;
    let state_proof = get_state_proof(nonce, 2, board_state_2, 0.into(), app_id);
    SingleGomoku::<T>::update_by_state(RawOrigin::Signed(palyers[0].clone()).into(), state_proof_2);

}

benchmarks! {
    _{ }

    app_initiate {
        let i in 0 .. 1000;
        let mut player1 = T::AccountId::default();
        let mut player2 = T::AccountId::default();
        player1 = account("player1", i, SEED);
        player2 = account("player2", i, SEED);
        let mut players = vec![];
        if player1 < player2 {
            players.push(player1.clone());
            players.push(player2.clone());
        } else {
            players.push(player2.clone());
            players.push(player1.clone());
        }

        let initiate_request = AppInitiateRequest {
            nonce: i as u128,
            players: players.clone(),
            timeout: 2.into(),
            min_stone_offchain: 5,
            max_stone_onchain: 5
        };
    }: _(RawOrigin::Signed(player1.clone()), initiate_request)

    update_by_state {
        let i in 0 .. 1000;
        let mut player1 = T::AccountId::default();
        let mut player2 = T::AccountId::default();
        player1 = account("player1", i, SEED);
        player2 = account("player2", i, SEED);
        let mut players = vec![];
        if player1 < player2 {
            players.push(player1.clone());
            players.push(player2.clone());
        } else {
            players.push(player2.clone());
            players.push(player1.clone());
        }

        let initiate_request = AppInitiateRequest {
            nonce: i as u128,
            players: players.clone(),
            timeout: 2.into(),
            min_stone_offchain: 5,
            max_stone_onchain: 5
        };
        SingleGomoku::<T>::app_initiate(RawOrigin::Signed(player1.clone()).into(), initiate_request.clone())?;
        let app_id = SingleGomoku::<T>::get_app_id(initiate_request.nonce, initiate_request.players.clone());
        let mut board_state = vec![0; 227];
        board_state[0] = 0;
        board_state[1] = 1;
        board_state[2] = 2;
        board_state[3] = 2;
        board_state[4] = 1;
        board_state[5] = 1;
        board_state[6] = 2;
        board_state[7] = 2;
        board_state[8] = 1;
        let state_proof = get_state_proof::<T>(i as u128, 1, board_state, 0.into(), app_id);
    }: _(RawOrigin::Signed(player1.clone()), state_proof)

    update_by_action {
        let i in 0 .. 1000;
        let mut player1 = T::AccountId::default();
        let mut player2 = T::AccountId::default();
        player1 = account("player1", i, SEED);
        player2 = account("player2", i, SEED);
        let mut players = vec![];
        if player1 < player2 {
            players.push(player1.clone());
            players.push(player2.clone());
        } else {
            players.push(player2.clone());
            players.push(player1.clone());
        }

        let initiate_request = AppInitiateRequest {
            nonce: i as u128,
            players: players.clone(),
            timeout: 2.into(),
            min_stone_offchain: 5,
            max_stone_onchain: 5
        };
        SingleGomoku::<T>::app_initiate(RawOrigin::Signed(player1.clone()).into(), initiate_request.clone())?;
        let app_id = SingleGomoku::<T>::get_app_id(initiate_request.nonce, initiate_request.players.clone());

        // place stone
        place_stone::<T>(app_id, i as u128, players.clone());

        let settle_finalized_time = SingleGomoku::<T>::get_settle_finalized_time(app_id).unwrap();
        System::<T>::set_block_number(settle_finalized_time + 1.into());
    }: _(RawOrigin::Signed(player1.clone()), app_id, vec![3, 12])

    finalize_on_action_timeout {
        let i in 0 .. 1000;
        let mut player1 = T::AccountId::default();
        let mut player2 = T::AccountId::default();
        player1 = account("player1", i, SEED);
        player2 = account("player2", i, SEED);
        let mut players = vec![];
        if player1 < player2 {
            players.push(player1.clone());
            players.push(player2.clone());
        } else {
            players.push(player2.clone());
            players.push(player1.clone());
        }

        let initiate_request = AppInitiateRequest {
            nonce: i as u128,
            players: players.clone(),
            timeout: 2.into(),
            min_stone_offchain: 5,
            max_stone_onchain: 5
        };
        SingleGomoku::<T>::app_initiate(RawOrigin::Signed(player1.clone()).into(), initiate_request.clone())?;
        let app_id = SingleGomoku::<T>::get_app_id(initiate_request.nonce, initiate_request.players.clone());

        let mut board_state = vec![0; 227];
        board_state[0] = 0; // winner
        board_state[1] = 2; // turn
        board_state[2] = 1; // (0, 0)
        board_state[3] = 1; // (0, 1)
        board_state[4] = 1; // (0, 2)
        board_state[5] = 1; // (0, 3)
        board_state[101] = 2;
        board_state[102] = 2;
        board_state[103] = 2;
        let state_proof = get_state_proof::<T>(i as u128, 3, board_state, 0.into(), app_id);
        SingleGomoku::<T>::update_by_state(RawOrigin::Signed(player1.clone()).into(), state_proof);
        let deadline = SingleGomoku::<T>::get_action_deadline(app_id).unwrap();
        System::<T>::set_block_number(deadline + 1.into());
    }: _(RawOrigin::Signed(player1.clone()), app_id)

    is_finalized {
        let i in 0 .. 1000;
        let mut player1 = T::AccountId::default();
        let mut player2 = T::AccountId::default();
        player1 = account("player1", i, SEED);
        player2 = account("player2", i, SEED);
        let mut players = vec![];
        if player1 < player2 {
            players.push(player1.clone());
            players.push(player2.clone());
        } else {
            players.push(player2.clone());
            players.push(player1.clone());
        }

        let initiate_request = AppInitiateRequest {
            nonce: i as u128,
            players: players.clone(),
            timeout: 2.into(),
            min_stone_offchain: 5,
            max_stone_onchain: 5
        };
        SingleGomoku::<T>::app_initiate(RawOrigin::Signed(player1.clone()).into(), initiate_request.clone())?;
        let app_id = SingleGomoku::<T>::get_app_id(initiate_request.nonce, initiate_request.players.clone());
        let mut board_state = vec![0; 227];
        board_state[0] = 2; // winner
        board_state[1] = 0; // turn
        let state_proof = get_state_proof::<T>(i as u128, 1, board_state, 0.into(), app_id);
        SingleGomoku::<T>::update_by_state(RawOrigin::Signed(player1.clone()).into(), state_proof);
    }: _(RawOrigin::Signed(player1.clone()), app_id)

    get_outcome {
        let i in 0 .. 1000;
        let mut player1 = T::AccountId::default();
        let mut player2 = T::AccountId::default();
        player1 = account("player1", i, SEED);
        player2 = account("player2", i, SEED);
        let mut players = vec![];
        if player1 < player2 {
            players.push(player1.clone());
            players.push(player2.clone());
        } else {
            players.push(player2.clone());
            players.push(player1.clone());
        }

        let initiate_request = AppInitiateRequest {
            nonce: i as u128,
            players: players.clone(),
            timeout: 2.into(),
            min_stone_offchain: 5,
            max_stone_onchain: 5
        };
        SingleGomoku::<T>::app_initiate(RawOrigin::Signed(player1.clone()).into(), initiate_request.clone())?;
        let app_id = SingleGomoku::<T>::get_app_id(initiate_request.nonce, initiate_request.players.clone());
        let mut board_state = vec![0; 227];
        board_state[0] = 2; // winner
        board_state[1] = 0; // turn
        let state_proof = get_state_proof::<T>(i as u128, 1, board_state, 0.into(), app_id);
        SingleGomoku::<T>::update_by_state(RawOrigin::Signed(player1.clone()).into(), state_proof);
    }: _(RawOrigin::Signed(player1.clone()), app_id, 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::*;
    use frame_support::assert_ok;

    #[test]
    fn app_initiate() {
        ExtBuilder::build().execute_with(|| {
            assert_ok!(test_benchmark_app_initiate::<TestRuntime>());
        })
    }

    #[test]
    fn update_by_state() {
        ExtBuilder::build().execute_with(|| {
            assert_ok!(test_benchmark_update_by_state::<TestRuntime>());
        });
    }

    #[test]
    fn update_by_action() {
        ExtBuilder::build().execute_with(|| {
            assert_ok!(test_benchmark_update_by_action::<TestRuntime>());
        });
    }

    #[test]
    fn finalize_on_action_timeout() {
        ExtBuilder::build().execute_with(|| {
            assert_ok!(test_benchmark_finalize_on_action_timeout::<TestRuntime>());
        });
    }

    #[test]
    fn is_finalized() {
        ExtBuilder::build().execute_with(|| {
            assert_ok!(test_benchmark_is_finalized::<TestRuntime>());
        });
    }

    #[test]
    fn get_outcome() {
        ExtBuilder::build().execute_with(|| {
            assert_ok!(test_benchmark_get_outcome::<TestRuntime>());
        });
    }
}