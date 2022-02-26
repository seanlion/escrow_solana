use solana_program::{
    pubkey::Pubkey,
    program_pack::{IsInitialized, Pack Sealed},
};
pub struct Escrow{
    pub is_initialized: bool,
    pub initializer_pubkey: Pubkey,
    //when Bob takes the trade, the escrow program can send tokens from the account at temp_token_account_pubkey to Bob's account.
    pub temp_token_account_pubkey: Pubkey,
    pub initializer_token_to_receive_account_pubkey: Pubkey,
    pub expected_amount: u64,
}

//Solana's version of Rust's Sized trait 
impl Sealed for Escrow {}

impl IsInitialized for Escrow {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
impl Pack for Escrow {
    //size of our Escrow struct type.
    //구조체 길이 : 1(bool) + 3 * 32(Pubkey) + 1 * 8(u64) = 105
    const LEN: usize = 105;
    //turns an array of u8 into an instance of the Escrow struct we defined above하는 함수
    fn unpack_from_slice(src: &[u8]) -> Result<Self,ProgramError> {
        let src = array_ref![src,0,Escrow::LEN];
        let (
            is_initialized,
            initializer_pubkey,
            temp_token_account_pubkey,
            initializer_token_to_receive_account_pubkey,
            expected_amount,
        ) = array_refs![src,1,32,32,32,8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Escrow {
            is_initialized,
            initializer_pubkey: Pubkey::new_from_array(*initializer_pubkey),
            temp_token_account_pubkey: Pubkey::new_from_array(*temp_token_account_pubkey),
            initializer_token_to_receive_account_pubkey: Pubkey::new_from_array(*initializer_token_to_receive_account_pubkey),
            expected_amount: u64::from_le_bytes(*expected_amount),
        })
    }

    // When we pack_into_slice, we already have an instance of an Escrow struct and now serialize it into the given dst slice.
    fn pack_into_slice(&self, dst: &mut[u8]){
        let dst = array_mut_ref![dst,0,Escrow::LEN];
        let(
            is_initialized_dst,
            initializer_pubkey_dst,
            temp_token_account_pubkey_dst,
            initializer_token_to_receive_account_pubkey_dst,
            expected_amount_dst,
        )  = mut_array_refs![dst,1,32,32,32,8];

        let Escrow {
            is_initialized,
            initializer_pubkey,
            temp_token_account_pubkey,
            initializer_token_to_receive_account_pubkey,
            expected_amount,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        initializer_pubkey_dst.copy_from_slice(initializer_pubkey.as_ref());
        temp_token_account_pubkey_dst.copy_from_slice(temp_token_account_pubkey.as_ref());
        initializer_token_to_receive_account_pubkey_dst.copy_from_slice(initializer_token_to_receive_account_pubkey.as_ref());
        *expected_amount_dst = expected_amount.to_le_bytes();
    }
}