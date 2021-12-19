use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_blog {
    use super::*;

    // Creation of a blog
    pub fn init_blog(ctx: Context<InitBlog>) -> ProgramResult {
        // Retrieving the accounts from context
        let blog_account = &mut ctx.accounts.blog_account;
        let genesis_post_account = &mut ctx.accounts.genesis_post_account;
        let authority = &mut ctx.accounts.authority;

        // Setting blog state
        blog_account.authority = authority.key();
        blog_account.current_post_key = genesis_post_account.key();

        Ok(())
    }

    // Defining the blog type
    #[derive(accounts)]
    pub struct InitBlog<'info> {
        #[account(init, payer = authority, space = 8 + 32 + 32)]
        pub blog_account: Account<'info, BlogState>,
        #[account(init, payer = authority, space = 8 + 32 + 32 + 32 + 32 + 8)]
        pub genesis_post_account: Account<'info, PostState>,
        pub authority: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[account]
    pub struct BlogState {
        pub current_post_key Pubkey,
        pub authority        Pubkey,
    }

    // User code
    pub fn signup_user(ctx: Context<SignupUser>, name: String, avatar: String) -> ProgramResult {
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        user_account.name = name;
        user_account.avatar = avatar;
        user_account.authority = authority.key();
        Ok(())
    }

    // Defining the user object
    #[derive(account)]
    pub struct SignupUser<'info> {
        #[account(init, payer = authority, space = 8 + 40 + 120 + 32)]
        pub user_account: Account<'info, UserState>,
        pub authority: Signup<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(account)]
    pub struct UserState{
        pub name: String,
        pub avatar: String,
        pub authority: PubKey,
    }
    
    // Program entry point
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
