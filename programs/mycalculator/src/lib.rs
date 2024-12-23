use anchor_lang::prelude::*;

declare_id!("C6Yp2iEtHhFdqhSs6Dd7maVaKLQ4fCkaarFqUs3LZVL4");

#[program]
pub mod mycalculator {
    use super::*;

    pub fn create(ctx: Context<Create>, init_message: String) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info>{
    // The ' symbol at the start indicates it's a lifetime annotation.
    #[account(init, payer=user, space=264)]
    pub calculator: Account<'info, Calculator>,
    #[account(mut)] // user is mutable
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Calculator{
    pub greeting: String,
    pub result: i64,
    pub remainder: i64
}