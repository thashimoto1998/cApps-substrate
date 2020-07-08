//! Single session app benchmarking

#![cfg(feature = "runtime-benchmarks")]
use super::*;
use frame_system::{RawOrigin, Module as System};
use frame_benchmarking::{benchmarks, account};
use crate::Module as SingleApp;

const SEED: u32 = 0;

fn get_state_proof<T: Trait>(
    nonce: u128, 
    seq: u128, 
    state: u8,
    timeout: T::BlockNumber, 
    app_id: T::Hash
) -> StateProof<T::BlockNumber, T::Hash> {
    let app_state = AppState {
        nonce: nonce,
        seq_num: seq,
        state: state,
        timeout: timeout,
        app_id: app_id,
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

        let initiate_request = AppInitiateRequest {
            nonce: i as u128,
            players: player,
            timeout: 2.into(),
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

        let initiate_request = AppInitiateRequest {
            nonce: i as u128,
            players: player.clone(),
            timeout: 2.into(),
        }; 
        SingleApp::<T>::app_initiate(RawOrigin::Signed(player1.clone()).into(), initiate_request.clone())?;
        let app_id = SingleApp::<T>::get_app_id(initiate_request.nonce, initiate_request.players.clone());
    
        let state_proof = get_state_proof::<T>(i as u128, 1, 2, 2.into(), app_id);
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

        let initiate_request = AppInitiateRequest {
            nonce: i as u128,
            players: player.clone(),
            timeout: 2.into()
        }; 
        SingleApp::<T>::app_initiate(RawOrigin::Signed(player1.clone()).into(), initiate_request.clone())?;
        let app_id = SingleApp::<T>::get_app_id(initiate_request.nonce, initiate_request.players.clone());
        let state_proof = get_state_proof::<T>(i as u128, 2, 5, 2.into(), app_id);
        SingleApp::<T>::update_by_state(RawOrigin::Signed(player1.clone()).into(), state_proof)?;
        let settle_finalized_time = SingleApp::<T>::get_settle_finalized_time(app_id).unwrap();
        System::<T>::set_block_number(settle_finalized_time + 1.into());
    }: _(RawOrigin::Signed(player1.clone()), app_id, 1)

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

        let initiate_request = AppInitiateRequest {
            nonce: i as u128,
            players: player.clone(),
            timeout: 2.into()
        }; 
        SingleApp::<T>::app_initiate(RawOrigin::Signed(player1.clone()).into(), initiate_request.clone())?;
        let app_id = SingleApp::<T>::get_app_id(initiate_request.nonce, initiate_request.players.clone());
        let state_proof = get_state_proof::<T>(i as u128, 1, 2, 2.into(), app_id);
        SingleApp::<T>::update_by_state(RawOrigin::Signed(player1.clone()).into(), state_proof)?;

        // advance blocknumber after action timeout
        System::<T>::set_block_number(5.into());
    }: _(RawOrigin::Signed(player1.clone()), app_id)

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

        let initiate_request = AppInitiateRequest {
            nonce: i as u128,
            players: player.clone(),
            timeout: 2.into()
        }; 
        SingleApp::<T>::app_initiate(RawOrigin::Signed(player1.clone()).into(), initiate_request.clone())?;
        let app_id = SingleApp::<T>::get_app_id(initiate_request.nonce, initiate_request.players.clone());
        let state_proof = get_state_proof::<T>(i as u128, 2, 2, 2.into(), app_id);
        SingleApp::<T>::update_by_state(RawOrigin::Signed(player1.clone()).into(), state_proof)?;
    }: _(RawOrigin::Signed(player1.clone()), app_id)

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

        let initiate_request = AppInitiateRequest {
            nonce: i as u128,
            players: player.clone(),
            timeout: 2.into()
        }; 
        SingleApp::<T>::app_initiate(RawOrigin::Signed(player1.clone()).into(), initiate_request.clone())?;
        let app_id = SingleApp::<T>::get_app_id(initiate_request.nonce, initiate_request.players.clone());
        let state_proof = get_state_proof::<T>(i as u128, 2, 2, 2.into(), app_id);
        SingleApp::<T>::update_by_state(RawOrigin::Signed(player1.clone()).into(), state_proof)?;
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
            assert_ok!(test_benchmaek_app_initiate::<TestRuntime>());
        });
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