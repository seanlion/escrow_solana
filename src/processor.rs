use solana_program::{
    account_info::{AccountInfo,next_account_info},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
    program_pack::{Pack,IsInitialized},
    sysvar::{rent::Rent,Sysvar},
    program::invoke
};

use crate::{instruction::EscrowInstruction, error::EscrowError, state::Escrow};

pub struct Processor;
impl Processor{
    //pass the reference to the slice holding the instruction_data from entrypoint.rs into the unpack function .
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = EscrowInstruction::unpack(instruction_data)?;

        match instruction  {
            EscrowInstruction::InitEscrow { amount } => {
                msg!("Instrunction: InitEscrow");
                Self::process_init_escrow(accounts,amount, program_id)
            }
        }
    }

    fn process_init_escrow(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        //create an iterator of accounts.
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;
        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        //transfer will fail if the temp_token_account is not owned by the token program.
        let temp_token_account = next_account_info(account_info_iter)?;
        let token_to_receive_account = next_account_info(account_info_iter)?;
        if *token_to_receive_account != spl_token::id(){
            return Err(ProgramError::IncorrectProgramId);
        }

        let escrow_account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
        if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()){
            return Err(EscrowError::NotRentExempt::into());
        }

        let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.try_borrow_data()?)?;
        if escrow_info.is_initialized(){
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        escrow_info.is_initialized = true;
        escrow_info.initializer_pubkey = *initializer.key;
        escrow_info.temp_token_account_pubkey = *temp_token_account.key;
        escrow_info.initializer_token_to_receive_account_pubkey = *token_to_receive_account.key;
        escrow_info.expected_amount = amount;

        //escrow_info를 serialize함.
        Escrow::pack(escrow_info, &mut escrow_account.try_borrow_mut_data()?)?;
        //create a PDA by passing in an array of seeds and the program_id into the find_program_address function
        let (pda, _bump_seed) = Pubkey::find_program_address(&[b"escrow"], program_id);
        
        let token_program = next_account_info(account_info_iter)?;
        let owner_change_ix = spl_token::instruction::set_authority(
            token_program.key,
            temp_token_account.key,
            Some(&pda),
            spl_token::instruction::AuthorityType::AccountOwner,
            initializer.key,
            &[&initializer.key],
        )?;
        msg!("Calling the token program to transfer token account ownership...");
        invoke(
            &owner_change_ix,
            &[
                temp_token_account.clone(),
                initializer.clone(),
                token_program.clone(),
            ]
        )?;
        Ok(())
    }
}