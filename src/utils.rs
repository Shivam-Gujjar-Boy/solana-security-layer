use solana_program::{
	sysvar::{clock::Clock, Sysvar},
	program_error::ProgramError,
};

pub fn get_clock() -> Result<Clock, ProgramError> {
	Clock::get()
}
