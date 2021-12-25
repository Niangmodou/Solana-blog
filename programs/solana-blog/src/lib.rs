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
    #[derive(Accounts)]
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
        pub current_post_key: Pubkey,
        pub authority:        Pubkey,
    }

    // User code
    // Defining the user object
    #[derive(Accounts)]
    pub struct SignupUser<'info> {
        #[account(init, payer = authority, space = 8 + 40 + 120 + 32)]
        pub user_account: Account<'info, UserState>,
        pub authority: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[account]
    pub struct UserState{
        pub name: String,
        pub avatar: String,
        pub authority: PubKey,
    }

    pub fn signup_user(ctx: Context<SignupUser>, name: String, avatar: String) -> ProgramResult {
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        user_account.name = name;
        user_account.avatar = avatar;
        user_account.authority = authority.key();

        Ok(())
    }

    pub fn update_user(ctx: Context<UpdateUser>, name: String, avatar: String) -> ProgramResult {
        let user_account = &mut ctx.accounts.user_account;

        user_account.name = name;
        user_account.avatar = avatar;

        Ok(())
    }
    #[derive(Accounts)]
    pub struct UpdateUser<'info> {
        #[account(mut, has_one = authority,)]
        pub user_account: Account<'info, UserState>,
        pub authority: Signer<'info>,
    }
    
    // CRUD of the post

    // Creating a new post
    pub fn create_post(ctx: Context<CreatePost>, title: String, content: String) -> ProgramResult {
        let blog_account = &mut ctx.accounts.blog_account;
        let post_account = &mut ctx.accounts.post_account;
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        post_account.title = title;
        post_account.content = content;
        post_account.user = user_account.key();
        post_account.pre_post_key = blog_account.current_post_key;

        blog_account.current_post_key = post_account.key();

        emit!(PostEvent {
            label: "CREATE".to_string(),
            post_id: post_account.key(),
            next_post_id: None
        });

        Ok(())
    }

    // Updating a post
    pub fn update_post(ctx: Context<UpdatePost>, title: String, content: String) -> ProgramResult {
        let post_account = &mut ctx.accounts.post_account;

        post_account.title = title;
        post_account.content = content;

        emit!(PostEvent {
            label: "UPDATE".to_string(),
            post_id: post_account.key(),
            next_post_id: None
        });

        Ok(())
    }

    // Event to let the client know the post is created
    #[event]
    pub struct PostEvent {
        pub label: String,                  // CREATE, UPDATE, DELETE
        pub post_id: Pubkey,                // The Created Post
        pub next_post_id: Option<Pubkey>,   // Emit the delete event
    }

    #[derive(Accounts)]
    pub struct CreatePost<'info> {
        #[account(init, payer = authority, space = 8 + 50 + 500 + 32 + 32 + 32)]
        pub post_account: Account<'info, PostState>,
        #[account(mut, has_one = authority)]
        pub user_account: Account<'info, UserState>,
        #[account(mut)]
        pub blog_account: Account<'info, BlogState>,
        pub authority: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[account]
    pub struct PostState {
        title: String,
        content: String,
        user: Pubkey,
        pub pre_post_key: Pubkey,
        pub authority: Pubkey,
    }

    
    // Program entry point
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
