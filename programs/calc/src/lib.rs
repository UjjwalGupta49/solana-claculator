use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod calc {
    use super::*;
    pub fn create(ctx: Context<Create>, init_message: String) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }
    pub fn add(ctx: Context<Addition>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        Ok(())
    }

    pub fn multiply(ctx: Context<Multiplication>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.claculator;
        calculator.result = num1 * num2;
        Ok(())
    }

    pub fn subtract(ctx: Context<Subtraction>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.claculator;
        calculator.result = num1 - num2;
        Ok(())
    }

    pub fn divide(ctx: Context<Division>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.claculator;
        calculator.result = num1/num2;
        calculator.remainder = num1%num2;
        Ok(())
}
}

#[account]
pub struct Calculator{
    pub greeting: String,
    pub result: i64,
    pub remainder: i64,
}


#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 8 + 64 + 64 + 64 + 64)] // init will create an account owned by this program
    pub calculator: Account<'info, Calculator>, // payer should alwys be defined when using init
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>
}
#[derive(Accounts)]
pub struct Subtraction<'info>{
    #[account(mut)]
    pub claculator: Account<'info, Calculator>
}
#[derive(Accounts)]
pub struct Multiplication<'info>{
    #[account(mut)]
    pub claculator: Account<'info, Calculator>
}
#[derive(Accounts)]
pub struct Division<'info>{
    #[account(mut)]
    pub claculator: Account<'info, Calculator>
}
// Anchor does not support floating values
// greeting, results, reaminer are the three feilds
// same account will be used for all the calculations, so we want our calculator account to be mutable

// solana accounts
// An account is not actually a wallet.
// accounts pay rent in form of lamports
// if the account runs out of lamports it is purged from chain
//  acounts with 2 years worth of rent attached are rent extempt and stay on chain forever

