#![cfg_attr(not(feature = "std"), no_std)]

//mod mock;

//#[cfg(test)]
//mod tests;

use codec::{Decode, Encode};
use frame_support::{
    decl_module, decl_storage, decl_event, decl_error, ensure,
    storage::StorageMap,
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::traits::{
    Hash, IdentifyAccount, 
    Member, Verify, Zero, AccountIdConversion, 
};
use sp_runtime::{ModuleId, RuntimeDebug, DispatchResult, DispatchError};
use sp_std::{prelude::*, vec::Vec};

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Encode, Decode, RuntimeDebug)]
pub struct AppInitiateRequest<AccountId, BlockNumber> {
    pub nonce: u128,
    pub players: Vec<AccountId>,
    pub timeout: BlockNumber,
    min_stone_offchain: u8,
    max_stone_onchain: u8,
}

pub type AppInitiateRequestOf<T> = AppInitiateRequest<
    <T as system::Trait>::AccountId,
    <T as system::Trait>::BlockNumber,
>;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Encode, Decode, RuntimeDebug)]
pub struct AppState<BlockNumber, Hash> {
    pub nonce: u128,
    pub seq_num: u128,
    pub state: GomokuState,
    pub timeout: BlockNumber,
    pub app_id: Hash,
}

pub type AppStateOf<T> = AppState<
    <T as system::Trait>::BlockNumber,
    <T as system::Trait>::Hash,
>;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Encode, Decode, RuntimeDebug)]
pub struct StateProof<BlockNumber, Hash, Signature> {
    pub app_state: AppState<BlockNumber, Hash>,
    pub sigs: Vec<Signature>,
}

pub type StateProofOf<T> = StateProof<
    <T as system::Trait>::BlockNumber,
    <T as system::Trait>::Hash,
    <T as Trait>::Signature,
>;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Encode, Decode, RuntimeDebug)]
pub enum AppStatus {
    Idle = 0,
    Settle = 1,
    Action = 2,
    Finalized = 3,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Encode, Decode, RuntimeDebug)]
pub struct AppInfo<AccountId, BlockNumber> {
    pub state: GomokuState,
    pub nonce: u128,
    pub players: Vec<AccountId>,
    pub seq_num: u128,
    pub timeout: BlockNumber,
    pub deadline: BlockNumber,
    pub status: AppStatus,
}

pub type AppInfoOf<T> = AppInfo<
    <T as system::Trait>::AccountId,
    <T as system::Trait>::BlockNumber,
>;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Encode, Decode, RuntimeDebug)]
pub enum StateKey {
    Turn = 0,
    Winner = 1,
    FullState = 2,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Encode, Decode, RuntimeDebug)]
struct GomokuState {
    bord_state: Vec<u8>,
    stone_num: u16,
    stone_num_onchain: u16,
    state_key: StateKey,
}

pub const SINGLE_GOMOKU_ID: ModuleId = ModuleId(*b"s_gomoku");

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Public: IdentifyAccount<AccountId = Self::AccountId>;
    type Signature: Verify<Signer = <Self as Trait>::Public> + Member + Decode + Encode; 
}

decl_storage! {
    trait Store for Module<T: Trait> as SingleSessionApp {
        pub SingleGomokuInfoMap get(fn app_info): 
            map hasher(blake2_128_concat) T::Hash => Option<AppInfoOf<T>>;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// board dimension is 15x15
        const BoardDimention: u8 = 15;


    }
}
