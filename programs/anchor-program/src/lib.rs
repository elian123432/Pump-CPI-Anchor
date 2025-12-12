use anchor_lang::prelude::*;

declare_id!("Hzof2DtGPz9JypHGCuRYU1UrHGNcz7SyQyy238CCwYK6");

// ref: https://github.com/solana-developers/program-examples/blob/104a0d36579bc903453fbc4c07157961d9d37897/basics/cross-program-invocation/anchor/programs/hand/src/lib.rs
declare_program!(pump_amm);
declare_program!(pump_fees);

//use pump_fees::accounts::FeeConfig;
pub use pump_fees::program::PumpFees;

pub use pump_amm::program::PumpAmm;
pub use pump_amm::accounts::*;

use anchor_lang::solana_program::{instruction::Instruction, program::invoke};
use anchor_spl::token_interface::{Mint, TokenAccount};
use anchor_spl::token::Token;
use anchor_spl::token_2022::Token2022;
use anchor_spl::associated_token::AssociatedToken;



#[program]
pub mod anchor_program {
    use super::*;

    pub fn buy_pump_swap_exact_out(ctx:Context<CheckedBuyAMM>, index:u16, creator:Pubkey, base_amount_out:u64, max_quote_amount_in:u64) -> Result<()> {

        

        let account_metas = vec![
            // use AccountMeta::new for write-accounts
            // use AccountMeta::new_readonly for write-accounts
            // args: Pubkey, isSigner

            AccountMeta::new(ctx.accounts.pool.key(), false),
            AccountMeta::new(ctx.accounts.user.key(), true),
            AccountMeta::new_readonly(ctx.accounts.global_config.key(), false),
            AccountMeta::new_readonly(ctx.accounts.base_mint.key(), false),
            AccountMeta::new_readonly(ctx.accounts.quote_mint.key(), false),

            AccountMeta::new(ctx.accounts.user_base_token_account.key(), false),
            AccountMeta::new(ctx.accounts.user_quote_token_account.key(), false),

            AccountMeta::new(ctx.accounts.pool_base_token_account.key(), false),
            AccountMeta::new(ctx.accounts.pool_quote_token_account.key(), false),

            AccountMeta::new_readonly(ctx.accounts.protocol_fee_recipient.key(), false),
            AccountMeta::new(ctx.accounts.protocol_fee_recipient_token_account.key(), false),

            AccountMeta::new_readonly(ctx.accounts.base_token_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.quote_token_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.associated_token_program.key(), false),

            AccountMeta::new_readonly(ctx.accounts.event_authority.key(), false),
            AccountMeta::new_readonly(ctx.accounts.program.key(), false),
            
            AccountMeta::new(ctx.accounts.coin_creator_vault_ata.key(), false),
            AccountMeta::new_readonly(ctx.accounts.coin_creator_vault_authority.key(), false),

            AccountMeta::new(ctx.accounts.global_volume_accumulator.key(), false),
            AccountMeta::new(ctx.accounts.user_volume_accumulator.key(), false),

            AccountMeta::new_readonly(ctx.accounts.fee_config.key(), false),
            AccountMeta::new_readonly(ctx.accounts.fee_program.key(), false),
    ];

    
    // you can get set them other ways
    // I looked at the idl and copied "discriminator": [], for the instruction that I want to CPI
    let instruction_discriminator: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];


    // pass in the instruction data
    // capazity is {[u8,8], u64, u64 }  => this will vary on each CPI 
    //  look at https://www.anchor-lang.com/docs/references/space for byte sizes
    let mut instruction_data = Vec::with_capacity(8+16);
    instruction_data.extend_from_slice(&instruction_discriminator);
    instruction_data.extend_from_slice(&base_amount_out.to_le_bytes());
    instruction_data.extend_from_slice(&max_quote_amount_in.to_le_bytes());

    // create instruction, use target program - NOT YOUR PROGRAM
    let instruction = Instruction {
        program_id: ctx.accounts.program.key(),
        accounts : account_metas,
        data:instruction_data,
    };

    // now invoke
    // you have to pass each account in the CORRECT ORDER
    // you can check order in idl
    // or on solscan: https://solscan.io/tx/2DPSr4uT1RPRBBFCyKn247RzvuTBs74QicJaGaXpEfK3UTPmsFcFoMZ54AGk7NydHBARez9qezusEe8x9WWkx7sw
    // simply check Instruction Details -> #5 Pump.Fun AMM:Buy (or the cpi that you want to invoke)
    invoke(&instruction, 
            &[
                ctx.accounts.pool.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.global_config.to_account_info(),

                ctx.accounts.base_mint.to_account_info(),
                ctx.accounts.quote_mint.to_account_info(),
                ctx.accounts.user_base_token_account.to_account_info(),
                ctx.accounts.user_quote_token_account.to_account_info(),

                ctx.accounts.pool_base_token_account.to_account_info(),
                ctx.accounts.pool_quote_token_account.to_account_info(),

                ctx.accounts.protocol_fee_recipient.to_account_info(),
                ctx.accounts.protocol_fee_recipient_token_account.to_account_info(),
                
             
                ctx.accounts.base_token_program.to_account_info(),
                ctx.accounts.quote_token_program.to_account_info(),

                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.associated_token_program.to_account_info(),

                ctx.accounts.event_authority.to_account_info(),
                ctx.accounts.program.to_account_info(),

                ctx.accounts.coin_creator_vault_ata.to_account_info(),
                ctx.accounts.coin_creator_vault_authority.to_account_info(),
                
                ctx.accounts.global_volume_accumulator.to_account_info(),
                ctx.accounts.user_volume_accumulator.to_account_info(),

                ctx.accounts.fee_config.to_account_info(),
                ctx.accounts.fee_program.to_account_info()
            ]
        )?;
    
    Ok(())

    }
}



// NOTE: you could pass each account as UncheckedAccount
// This would still work since the CPI accesses and deserializes the account data
// HOWEVER if you want to access the accounts inside YOUR program you need to deserialize (eg not use UncheckedAccount)
#[derive(Accounts)]
#[instruction(index:u16, creator:Pubkey)]
pub struct CheckedBuyAMM<'info> {

    #[account(mut,
            seeds = [b"pool", index.to_le_bytes().as_ref(), creator.key().as_ref(), base_mint.key().as_ref(), quote_mint.key().as_ref()],
            bump=pool.pool_bump,
            seeds::program = program.key(), 
    )]
    pub pool: Account<'info, Pool>,
    

    // writable, signer, fee payer
    #[account(mut,
    )]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"global_config"],
        bump,
        seeds::program = program.key(),  
    )]
    pub global_config: Box<Account<'info, GlobalConfig>>,

    #[account(
        //address = pool.base_mint,
    )]
    /// CHECK: Validated via address constraint
    pub base_mint: InterfaceAccount<'info,Mint>,
    
     ///relation pool
    #[account(
        //address = pool.quote_mint,
    )]
    pub quote_mint: InterfaceAccount<'info, Mint>,
     
    #[account(
        mut,
        //associated_token::mint = base_mint.key(),
        //associated_token::authority = user.key(),
        //associated_token::token_program = base_token_program.key(),
    )]
    pub user_base_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut,
        //associated_token::mint = quote_mint.key(),
        //associated_token::authority = user.key(),
        //associated_token::token_program = quote_token_program.key(),
    )]
    pub user_quote_token_account: InterfaceAccount<'info, TokenAccount>,

    // writable
    /// CHECK: pool token account of base
    #[account(mut,
        //address=pool.pool_base_token_account
    )]
    pub pool_base_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut,
        //address=pool.pool_quote_token_account
    )]
    pub pool_quote_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: fee recepient
    #[account()]
    pub protocol_fee_recipient: UncheckedAccount<'info>,

    // writable
    /// CHECK: fee recepients token account
    #[account(mut,
    )]
    pub protocol_fee_recipient_token_account: InterfaceAccount<'info, TokenAccount>,
    //program
    #[account()]
    pub base_token_program: Program<'info, Token2022>,  

    //program
    #[account()]
    pub quote_token_program: Program<'info, Token>,

    // system program
    #[account()]
    pub system_program: Program<'info, System>,

   
    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,

     /// CHECK: has some pda that idk
    #[account()]
    pub event_authority: UncheckedAccount<'info>,
    
    // CHECK: needs to be passed into the off-chain client -> will error if not
    #[account()]
    pub program: Program<'info, PumpAmm>, 
    

    /// CHECK: is vault atat
    #[account(mut,
        //associated_token::mint = quote_mint.key(),
        //associated_token::authority = coin_creator_vault_authority.key(),
        //associated_token::token_program = associated_token_program.key(),
    )]
    //pub coin_creator_vault_ata: InterfaceAccount<'info, TokenAccount>,
    pub coin_creator_vault_ata: UncheckedAccount<'info>,
    
    /// CHECK: is vault authority
    #[account(  
    )]
    pub coin_creator_vault_authority: UncheckedAccount<'info>,
    
    // Box or you hit 4kb limit
    #[account(mut,
    )]
    pub global_volume_accumulator: Box<Account<'info, GlobalVolumeAccumulator>>,
    
    /// CHECK needs to be custom found/derived off-chain 
    #[account(mut,
        seeds = [b"user_volume_accumulator", user.key().as_ref()],
        bump, // you can also get bump offchain and pass in as instruction??
        seeds::program = program.key(),  
    )]
    pub user_volume_accumulator: UncheckedAccount<'info>,
   
    /// CHECK: dont know from which program
    #[account(
        //seeds = [b"fee_config", fee_program.key().as_ref()],
        //bump=fee_config.bump, // use real bump
        //seeds::program = fee_program.key(),  // dont use since its not derived from fee_program
    )]
    pub fee_config: UncheckedAccount<'info>,
    //pub fee_config: Account<'info, FeeConfig>,

     /// CHECK: is fee program  pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ
    #[account()]
    pub fee_program: Program<'info, PumpFees>, // should be program
    // should be 23 accounts in total
   
}