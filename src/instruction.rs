use std::convert::TryInto;
use solana_program::program_error::ProgramError;

use crate::error::EscrowError::InvalidInstruction;

pub enum EscrowInstruction{
    InitEscrow {
        amount: u64
    }
}

impl EscrowInstruction {
    //unpack expects a reference to a slice of u8. 
    pub fn unpack(input: &[u8]) -> Result<Self,ProgramError>{
        //It looks at the first byte (=tag) to determine how to decode the rest (=rest) of the slice.
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64,ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount) 
    }

}