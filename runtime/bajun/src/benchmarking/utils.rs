pub use crate::{AccountId, Balances, Runtime, BAJUN};
use frame_support::dispatch::RawOrigin;
pub use parity_scale_codec::Encode;
use sp_runtime::traits::{AccountIdConversion, StaticLookup};

pub fn get_vesting_account() -> AccountId {
	crate::TreasuryPalletId::get().into_account_truncating()
}

pub fn lookup_of_account(
	who: AccountId,
) -> <<Runtime as frame_system::Config>::Lookup as StaticLookup>::Source {
	<Runtime as frame_system::Config>::Lookup::unlookup(who)
}

pub fn set_balance(account: AccountId, schedule_amount: u128) {
	Balances::force_set_balance(RawOrigin::Root.into(), account.into(), schedule_amount).unwrap();
}

#[cfg(test)]
pub mod tests {
	use sp_runtime::BuildStorage;

	pub fn new_test_ext() -> sp_io::TestExternalities {
		frame_system::GenesisConfig::<crate::Runtime>::default()
			.build_storage()
			.unwrap()
			.into()
	}
}
