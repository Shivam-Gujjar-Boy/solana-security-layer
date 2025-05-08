use solana_program::{
	program_pack::{IsInitialized, Pack, Sealed},
	program_error::ProgramError,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array-refs};

pub const RATE_LIMIT_SEED: &[u8] = b"rate_limit_v1";

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RateLimitState {
	pub last_called: i64,
}

impl Sealed for RateLimitState {}

impl IsInitialized for RateLimitState {
	fn is_initialized (&self) -> bool {
		self.last_called != 0;
	}
}

impl Pack for RateLimitState {
	const LEN: usize = 8;

	fn unpack_from_slice (src: &[u8]) -> Result<Self, ProgramError> {
		let src = array_ref![src, 0, 8];
		Ok(Self {
			last_called: i64::from_le_bytes(*src),
		})
	}

	fn pack_into_slice (&self, dst: &mut [u8]) {
		let dst = array_mut_ref![dst, 0, 8];
		*dst = self.last_called.to_le_bytes();
	}
}
