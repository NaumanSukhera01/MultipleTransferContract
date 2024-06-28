use anchor_lang::prelude::*;

declare_id!("Nqq9omvDy79f3hjiMt7Uc5wdSahuruvu8cyDSxs8sEs");
#[program]
pub mod sol{
    use super::*;
    pub fn init(ctx:Context<Init>)->Result<()>
    {
        Ok(())
    }
    pub fn transfer_sol(ctx: Context<TransferSol>, amount: u64, recipients: Vec<Pubkey>) -> Result<()> {
        msg!("Starting transfer_sol");
        
        let total_recipients = recipients.len();
        if total_recipients == 0 {
            msg!("No recipients provided");
            return Err(ProgramError::InvalidArgument.into());
        }

        msg!("Total recipients: {}", total_recipients);
        let amount_per_recipient = amount / (total_recipients as u64);
        msg!("Amount per recipient: {}", amount_per_recipient);

        for (i, &recipient) in recipients.iter().enumerate() {
            msg!("Processing recipient {}", i);
            
            if ctx.remaining_accounts.len() <= i {
                msg!("Not enough remaining accounts");
                return Err(ProgramError::NotEnoughAccountKeys.into());
            }

            let recipient_account = &ctx.remaining_accounts[i];
            
            if recipient_account.key() != recipient {
                msg!("Recipient account mismatch");
                return Err(ProgramError::InvalidAccountData.into());
            }

            // let from_lamports = ctx.accounts.from.to_account_info().;
            // if from_lamports < amount_per_recipient {
            //     msg!("Insufficient funds");
            //     return Err(ProgramError::InsufficientFunds.into());
            // }

            **ctx.accounts.from.to_account_info().try_borrow_mut_lamports()? -= amount_per_recipient;
            **recipient_account.try_borrow_mut_lamports()? += amount_per_recipient;

            msg!("Transferred {} to recipient {}", amount_per_recipient, recipient);
        }

        msg!("Transfer completed successfully");
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Init<'info>{
    #[account(init,payer=payer,space=8,seeds=[b"t"],bump)]
   pub from: Account<'info,Pda>,
   #[account(mut)]
   pub payer:Signer<'info>,
   pub system_program: Program<'info, System>,


}
#[derive(Accounts)]
pub struct TransferSol<'info> {
    #[account(mut,seeds=[b"t"],bump)]
    from: Account<'info,Pda>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct Pda{

}
