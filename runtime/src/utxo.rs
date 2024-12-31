#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use parity_scale_codec::{Decode, Encode};
use sp_core::{
	crypto::Public as _,
	H256,
	H512,
	sr25519::{Public, Signature},
};
use sp_std::{
    collections::btree_map::BTreeMap,
    vec::Vec
};
use sp_runtime::{
	traits::{BlakeTwo256, Hash, SaturatedConversion},
	transaction_validity::{TransactionLongevity, ValidTransaction},
};
use scale_info::TypeInfo;
use super::{block_author::BlockAuthor, issuance::Issuance};

pub type Value = u128;

pub use pallet::*;

/// Single transaction to be dispatched
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, Hash, Debug)]
pub struct Transaction {
	/// UTXOs to be used as inputs for current transaction
	pub inputs: Vec<TransactionInput>,

	/// UTXOs to be created as a result of current transaction dispatch
	pub outputs: Vec<TransactionOutput>,
}

/// Single transaction input that refers to one UTXO
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, Hash, Debug)]
pub struct TransactionInput {
	/// Reference to an UTXO to be spent
	pub outpoint: H256,

	/// Proof that transaction owner is authorized to spend referred UTXO &
	/// that the entire transaction is untampered
	pub sigscript: H512,
}

/// Single transaction output to create upon transaction dispatch
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, Hash, Debug, TypeInfo)]
pub struct TransactionOutput {
	/// Value associated with this output
	pub value: Value,

	/// Public key associated with this output. In order to spend this output
	/// owner must provide a proof by hashing the whole `Transaction` and
	/// signing it with a corresponding private key.
	pub pubkey: H256,
}

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
		/// https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_runtime_types/index.html
        type RuntimeEvent: From<Event<Self>>
    		+ IsType<<Self as frame_system::Config>::RuntimeEvent>;

        // /// A source to determine the block author
        // type BlockAuthor: BlockAuthor;

        /// A source to determine the issuance portion of the block reward
        type Issuance: Issuance<BlockNumberFor<Self>, Value>;
    }


    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Total reward value to be redistributed among authorities.
    /// It is accumulated from transactions during block execution
    /// and then dispersed to validators on block finalization.
    #[pallet::storage]
    #[pallet::getter(fn total_reward)]
    pub type TotalReward<T: Config> =
        StorageValue<_, Value, ValueQuery>;
     
    /// All valid unspent transaction outputs are stored in this map.
    /// Initial set of UTXO is populated from the list stored in genesis.
    /// We use the identity hasher here because the cryptographic hashing is
    /// done explicitly.
    /// Mapping from `hash_of(transaction, index)` to `TransactionOutput`
    #[pallet::storage]
    #[pallet::getter(fn utxo_store)]
    pub type UtxoStore<T: Config> = StorageMap<
            Hasher = Identity,
            Key=H256,
            Value=TransactionOutput,
            QueryKind=OptionQuery
        >;

    
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub _ph_data: PhantomData<T>,
        pub genesis_utxos: Vec<TransactionOutput>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            for utxo in self.genesis_utxos.iter() {
                UtxoStore::<T>::insert(&BlakeTwo256::hash_of(&utxo), utxo);
            }
        }
    }

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                _ph_data: Default::default(),
                genesis_utxos: vec![],
            }
        }
    }
        
    /// Pallets use events to inform users when important changes are made.
	/// https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {

    }

    /// Errors inform users that something went wrong.
	/// https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error
	#[pallet::error]
	pub enum Error<T> {
    }
}