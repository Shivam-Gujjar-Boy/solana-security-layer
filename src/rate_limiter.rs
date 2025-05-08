use crate::state::{RateLimitState, RATE_LIMIT_SEED};
use crate::utils;
use solana_program::{
	account_info::{next_account_info, AccountInfo},
	entrypoint::ProgramResult,
	program_error::ProgramError,
	pubkey::Pubkey,
	msg,
};

pub fn handle_rate_limited_call(
	program_id: &Pubkey,
	accounts: &[AccountInfo],
) -> ProgramResult {
	let user = &accounts[0];
	let rate_account = &account[1];

	let clock = utils::get_clock()?;
	
	let mut state = RateLimitState::unpack_unchecked(&rate_account.try_borrow_data()?)?;
	if state.last_called > 0 && clock.unix_timestamp - state.last_called < 60 {
		msg!("Rate limit exceeded. Try again later.");
		return Err(ProgramError::Custom(6000));
	}

	state.last_called = clock.unix_timestamp;
	state.pack(&mut rate_account.try_borrow_mut_data()?)?;

	Ok(())
}
