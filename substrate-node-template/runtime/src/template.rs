#![cfg_attr(not(feature = "std"), no_std)]


use sp_std::convert::TryInto;
use scale_info::TypeInfo;


pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	// use frame_system::Event;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	// Configuring the pallet
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// Because this pallet emit events, it depends on the runtime definition of an event
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	// pallet uses events to inform users when important changes are being made
	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		//Event emitted when a claim has been created
		ClaimCreated { who: T::AccountId, claim: T::Hash },
		//Event emitted when a claim is revoked by the owner
		ClaimRevoked { who: T::AccountId, claim: T::Hash },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The claim already exists
		AlreadyClaimed,
		/// The claim doesn't exist to revoke
		NoSuchClaim,
		/// The claim is owned by another account
		NotClaimOwner,
	}

	#[pallet::storage]
	pub(super) type Claims<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, (T::AccountId, T::BlockNumber)>;

	// Dispatchable functions allow users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
			//Check that the extrinsic was signed and get the signer.
			// this function will return an error if the extrinsic is not signed

			let sender = ensure_signed(origin)?;

			// Verify that the specified claim has not already been stored.
			ensure!(!Claims::<T>::contains_key(&claim), Error::<T>::AlreadyClaimed);

			// Get the block number from the FRAME System Pallet;
			let current_block = <frame_system::Pallet<T>>::block_number();

			// Store the claim with the sender and block number
			Claims::<T>::insert(&claim, (&sender, current_block));

			// Emit an event that the claim was created;
			Self::deposit_event(Event::ClaimCreated {who: sender, claim});

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed
			let sender = ensure_signed(origin)?;

			// Get the owner of the claim, if note return an error.
			let (owner, _) = Claims::<T>::get(&claim).ok_or(Error::<T>::NoSuchClaim)?;

			// Verify that sender of the current call is the claim owner
			ensure!(sender == owner,  Error::<T>::NotClaimOwner);

			// Remove claim from storage
			Claims::<T>::remove(&claim);

			// Emit an event that the claim was erased
			Self::deposit_event(Event::ClaimRevoked { who: sender, claim});
			Ok(())
		}
	}
}



// use codec::{Decode, Encode};
//
// // use pallet_aura::Config;
// /// A runtime module template with necessary imports
//
// /// Feel free to remove or edit this file as needed.
// /// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
// /// If you remove this file, you can remove those references
//
//
// /// For more guidance on Substrate modules, see the example module
// /// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs
//
// use frame_support::{decl_module, decl_storage, decl_event, dispatch::{DispatchResult, Vec}, ensure};
// use frame_system::ensure_signed;
//
// /// The module's configuration trait.
// pub trait Config: frame_system::Config {
// 	// TODO: Add other types and constants required configure this module.
//
// 	/// The overarching event type.
// 	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
// }
//
// // This module's storage items.
// decl_storage! {
// 	trait Store for Module<T: Config> as TemplateModule {
// 		// Just a dummy storage item.
// 		// Here we are declaring a StorageValue, `Something` as a Option<u32>
// 		// `get(something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
// 		Something get(something): Option<u32>;
// 	}
// }
//
// // The module's dispatchable functions.
// decl_module! {
// 	/// The module declaration.
// 	pub struct Module<T: Config> for enum Call where origin: T::Origin {
// 		// Initializing events
// 		// this is needed only if you are using events in your module
// 		fn deposit_event() = default;
//
// 		// Just a dummy entry point.
// 		// function that can be called by the external world as an extrinsics call
// 		// takes a parameter of the type `AccountId`, stores it and emits an event
// 		pub fn do_something(origin, something: u32) -> Result {
// 			// TODO: You only need this if you want to check it was signed.
// 			let who = ensure_signed(origin)?;
//
// 			// TODO: Code to execute when something calls this.
// 			// For example: the following line stores the passed in u32 in the storage
// 			Something::put(something);
//
// 			// here we are raising the Something event
// 			Self::deposit_event(RawEvent::SomethingStored(something, who));
// 			Ok(())
// 		}
// 	}
// }
//
// decl_event!(
// 	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
// 		// Just a dummy event.
// 		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
// 		// To emit this event, we call the deposit funtion, from our runtime funtions
// 		SomethingStored(u32, AccountId),
// 	}
// );
//
// /// tests for this module
// #[cfg(test)]
// mod tests {
// 	// use pallet_aura::Config;
// 	use super::*;
//
// 	use frame_support::{assert_ok, assert_noop, impl_outer_origin, parameter_types, weights::Weight};
// 	use sp_runtime::{testing::Header, traits::IdentityLookup, Perbill};
// 	use sp_core::testing::{KeyStore, SR25519};
// 	use sp_core::traits::KeystoreExt;
//
// 	impl_outer_origin! {
// 		pub enum Origin for Test {}
// 	}
//
// 	// For testing the module, we construct most of a mock runtime. This means
// 	// first constructing a configuration type (`Test`) which `impl`s each of the
// 	// configuration traits of modules we want to use.
// 	#[derive(Clone, Eq, PartialEq)]
// 	pub struct Test;
// 	parameter_types! {
// 		pub const BlockHashCount: u64 = 250;
// 		pub const MaximumBlockWeight: Weight = 1024;
// 		pub const MaximumBlockLength: u32 = 2 * 1024;
// 		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
// 	}
// 	impl frame_system::Config for Test {
// 		type Origin = Origin;
// 		type Call = ();
// 		type Index = u64;
// 		type BlockNumber = u64;
// 		type Hash = H256;
// 		type Hashing = BlakeTwo256;
// 		type AccountId = u64;
// 		type Lookup = IdentityLookup<Self::AccountId>;
// 		type Header = Header;
// 		type Event = ();
// 		type BlockHashCount = BlockHashCount;
// 		type Version = ();
// 		type BaseCallFilter = ();
// 		type BlockWeights = ();
// 		type BlockLength = ();
// 		type DbWeight = ();
// 		type PalletInfo = ();
// 		type AccountData = ();
// 		type OnNewAccount = ();
// 		type OnKilledAccount = ();
// 		type SystemWeightInfo = ();
// 		type SS58Prefix = ();
// 		type OnSetCode = ();
// 		type MaxConsumers = ();
// 	}
// 	impl Config for Test {
// 		type Event = ();
// 	}
// 	type TemplateModule = Module<Test>;
//
// 	// This function basically just builds a genesis storage key/value store according to
// 	// our desired mockup.
// 	fn new_test_ext() -> runtime_io::TestExternalities {
// 		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
// 	}
//
// 	#[test]
// 	fn it_works_for_default_value() {
// 		new_test_ext().execute_with(|| {
// 			// Just a dummy test for the dummy funtion `do_something`
// 			// calling the `do_something` function with a value 42
// 			assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
// 			// asserting that the stored value is equal to what we stored
// 			assert_eq!(TemplateModule::something(), Some(42));
// 		});
// 	}
// }


