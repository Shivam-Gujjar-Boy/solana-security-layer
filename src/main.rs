use solana_program::{
	account_info::{next_account_info, AccountInfo},
	entrypoint,
	entrypoint::ProgramResult,
	pubkey::Pubkey,
	msg,
};

mod rate_limiter;
mod state;
mod utils;

entrypoint!(process_instruction);

fn process_instruction(
	program_id: &Pubkey,
	accounts: &[AccountInfo],
	instruction_data: &[u8],
) -> ProgramResult {
	let instruction_tag = instruction_data[0];

	match instruction_tag {
		0 => {
			msg!("Handling rate-limited function");
			rate_limiter::handle_rate_limited_call(program_id, accounts)?;
		}
		_ => {
			msg!("Unknown Instruction");
		}
	}

	Ok(())
}
