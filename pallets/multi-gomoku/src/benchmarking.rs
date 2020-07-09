//! Multi gomoku benchmarking

#![cfg(feature = "runtime-benchmarks")]
use super::*;
use frame_system::{RawOrigin, Module as System};
use frame_benchmarking::{benchmarks, account};
use crate::Module as MultiGomoku;
use sp_std::vec::Vec;

const SEED: u32 = 0;
const NONE: u8 = 0;
const BLACK: u8 = 1;
const WHITE: u8 = 2;
const BLACK_PLAYER_ID_1: u8 = 2;
const BLACK_PLAYER_ID_2: u8 = 1;

fn get_state_proof<T: Trait>(
    seq: u128,
    board_state: Vec<u8>,
    timeout: T::BlockNumber,
    app_id: T::Hash
) -> StateProof<T::BlockNumber, T::Hash> {
    let app_state = AppState {
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

fn place_stone<T: Trait>(
    app_id: T::Hash,
    players: Vec<T::AccountId>,
    nonce: u128,
) {
    let mut board_state_1 = vec![0; 228];
    board_state_1[0] = NONE;
    board_state_1[1] = BLACK;
    board_state_1[2] = BLACK_PLAYER_ID_1;
    board_state_1[3] = WHITE;
    board_state_1[4] = WHITE;
    board_state_1[5] = BLACK;
    board_state_1[6] = BLACK;
    board_state_1[7] = WHITE;
    board_state_1[8] = WHITE;
    board_state_1[9] = BLACK;
    let state_proof = get_state_proof::<T>(1, board_state_1, 2.into(), app_id);
    MultiGomoku::<T>::update_by_state(RawOrigin::Signed(players[0].clone()).into(), state_proof);

    let mut board_state_2 = vec![0; 228];
    board_state_2[0] = NONE;
    board_state_2[1] = WHITE;
    board_state_2[2] = BLACK_PLAYER_ID_1;
    board_state_2[3] = BLACK;
    board_state_2[4] = BLACK;
    board_state_2[5] = BLACK;
    board_state_2[6] = BLACK;
    board_state_2[101] = WHITE;
    board_state_2[102] = WHITE;
    board_state_2[103] = WHITE;
    let state_proof = get_state_proof::<T>(2, board_state_2, 2.into(), app_id);
    MultiGomoku::<T>::update_by_state(RawOrigin::Signed(players[0].clone()).into(), state_proof);
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
            player_num: 2,
            timeout: 2.into(),
            min_stone_offchain: 5,
            max_stone_onchain: 5
        };
    }: _(RawOrigin::Signed(players[0].clone()), initiate_request)

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
            player_num: 2,
            timeout: 2.into(),
            min_stone_offchain: 5,
            max_stone_onchain: 5
        };
        MultiGomoku::<T>::app_initiate(RawOrigin::Signed(players[0].clone()).into(), initiate_request.clone())?;

        let app_id = MultiGomoku::<T>::get_app_id(initiate_request.nonce, initiate_request.players.clone());
        let mut board_state = vec![0; 228];
        board_state[0] = NONE;
        board_state[1] = BLACK;
        board_state[2] = BLACK_PLAYER_ID_1;
        board_state[3] = WHITE;
        board_state[4] = WHITE;
        board_state[5] = BLACK;
        board_state[6] = BLACK;
        board_state[7] = WHITE;
        board_state[8] = WHITE;
        board_state[9] = BLACK;
        let state_proof = get_state_proof::<T>(1, board_state, 2.into(), app_id);
    }: _(RawOrigin::Signed(players[0].clone()), state_proof)

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
            player_num: 2,
            timeout: 2.into(),
            min_stone_offchain: 5,
            max_stone_onchain: 5
        };
        MultiGomoku::<T>::app_initiate(RawOrigin::Signed(players[0].clone()).into(), initiate_request.clone())?;

        let app_id = MultiGomoku::<T>::get_app_id(initiate_request.nonce, initiate_request.players.clone());
        // place stone
        place_stone::<T>(app_id, players.clone(), i as u128);

        let settle_finalized_time = MultiGomoku::<T>::get_settle_finalized_time(app_id).unwrap();
        System::<T>::set_block_number(settle_finalized_time + 1.into());
    }: _(RawOrigin::Signed(players[0].clone()), app_id, vec![3, 12])

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
            player_num: 2,
            timeout: 2.into(),
            min_stone_offchain: 5,
            max_stone_onchain: 5
        };
        MultiGomoku::<T>::app_initiate(RawOrigin::Signed(player1.clone()).into(), initiate_request.clone())?;

        let app_id = MultiGomoku::<T>::get_app_id(initiate_request.nonce, initiate_request.players.clone());
        let mut board_state = vec![0; 228];
        board_state[0] = NONE;
        board_state[1] = WHITE;
        board_state[2] = BLACK_PLAYER_ID_2;
        board_state[3] = BLACK;
        board_state[4] = BLACK;
        board_state[5] = BLACK;
        board_state[6] = BLACK;
        board_state[101] = WHITE;
        board_state[102] = WHITE;
        board_state[103] = WHITE;
        let state_proof = get_state_proof::<T>(1, board_state, 2.into(), app_id);
        MultiGomoku::<T>::update_by_state(RawOrigin::Signed(players[0].clone()).into(),  state_proof)?;
        
        let settle_finalized_time = MultiGomoku::<T>::get_settle_finalized_time(app_id).unwrap();
        System::<T>::set_block_number(settle_finalized_time + 1.into());

        MultiGomoku::<T>::update_by_action(RawOrigin::Signed(players[1].clone()).into(), app_id, vec![3, 12])?;

        let deadline = MultiGomoku::<T>::get_action_deadline(app_id).unwrap();
        System::<T>::set_block_number(deadline + 1.into());
    }: _(RawOrigin::Signed(players[0].clone()), app_id)

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
            player_num: 2,
            timeout: 2.into(),
            min_stone_offchain: 5,
            max_stone_onchain: 5
        };
        MultiGomoku::<T>::app_initiate(RawOrigin::Signed(players[0].clone()).into(), initiate_request.clone())?;

        let app_id = MultiGomoku::<T>::get_app_id(initiate_request.nonce, initiate_request.players.clone());
        let mut board_state = vec![0; 228];
        board_state[0] = NONE;
        board_state[1] = WHITE;
        board_state[2] = BLACK_PLAYER_ID_2;
        board_state[3] = BLACK;
        board_state[4] = BLACK;
        board_state[5] = BLACK;
        board_state[6] = BLACK;
        board_state[101] = WHITE;
        board_state[102] = WHITE;
        board_state[103] = WHITE;
        let state_proof = get_state_proof::<T>(1, board_state, 2.into(), app_id);
        MultiGomoku::<T>::update_by_state(RawOrigin::Signed(players[0].clone()).into(),  state_proof)?;
        
        let settle_finalized_time = MultiGomoku::<T>::get_settle_finalized_time(app_id).unwrap();
        System::<T>::set_block_number(settle_finalized_time + 1.into());

        MultiGomoku::<T>::update_by_action(RawOrigin::Signed(players[1].clone()).into(), app_id, vec![3, 12])?;

        let deadline = MultiGomoku::<T>::get_action_deadline(app_id).unwrap();
        System::<T>::set_block_number(deadline + 1.into());
        MultiGomoku::<T>::finalize_on_action_timeout(RawOrigin::Signed(players[0].clone()).into(), app_id)?;
    }: _(RawOrigin::Signed(players[0].clone()), app_id)

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
            player_num: 2,
            timeout: 2.into(),
            min_stone_offchain: 5,
            max_stone_onchain: 5
        };
        MultiGomoku::<T>::app_initiate(RawOrigin::Signed(players[0].clone()).into(), initiate_request.clone())?;

        let app_id = MultiGomoku::<T>::get_app_id(initiate_request.nonce, initiate_request.players.clone());
        let mut board_state = vec![0; 228];
        board_state[0] = NONE;
        board_state[1] = WHITE;
        board_state[2] = BLACK_PLAYER_ID_2;
        board_state[3] = BLACK;
        board_state[4] = BLACK;
        board_state[5] = BLACK;
        board_state[6] = BLACK;
        board_state[101] = WHITE;
        board_state[102] = WHITE;
        board_state[103] = WHITE;
        let state_proof = get_state_proof::<T>(1, board_state, 2.into(), app_id);
        MultiGomoku::<T>::update_by_state(RawOrigin::Signed(players[0].clone()).into(),  state_proof)?;
        
        let settle_finalized_time = MultiGomoku::<T>::get_settle_finalized_time(app_id).unwrap();
        System::<T>::set_block_number(settle_finalized_time + 1.into());

        MultiGomoku::<T>::update_by_action(RawOrigin::Signed(players[1].clone()).into(), app_id, vec![3, 12])?;

        let deadline = MultiGomoku::<T>::get_action_deadline(app_id).unwrap();
        System::<T>::set_block_number(deadline + 1.into());
        MultiGomoku::<T>::finalize_on_action_timeout(RawOrigin::Signed(players[0].clone()).into(), app_id)?;
    }: _(RawOrigin::Signed(players[0].clone()), app_id, 2)
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
        })
    }

    #[test]
    fn update_by_action() {
        ExtBuilder::build().execute_with(|| {
            assert_ok!(test_benchmark_update_by_action::<TestRuntime>());
        })
    }

    #[test]
    fn finalize_on_action_timeout() {
        ExtBuilder::build().execute_with(|| {
            assert_ok!(test_benchmark_finalize_on_action_timeout::<TestRuntime>());
        })
    }

    #[test]
    fn is_finalized() {
        ExtBuilder::build().execute_with(|| {
            assert_ok!(test_benchmark_is_finalized::<TestRuntime>());
        })
    }

    #[test]
    fn get_outcome() {
        ExtBuilder::build().execute_with(|| {
            assert_ok!(test_benchmark_get_outcome::<TestRuntime>());
        })
    }
}