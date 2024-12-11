// Imports
use anchor_lang::prelude::*;

// ID du program
declare_id!("");

#[program]
pub mod todolist {

    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>, nickname: String) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.user_pubkey = *ctx.accounts.signer.key;
        user.nickname = nickname;
        user.todo_count = 0;

        Ok(())
    }

    pub fn initialize_todo(
        ctx: Context<InitializeTodo>,
        todo_count_index: u64,
        description: String,
    ) -> Result<()> {
        if description.len() > 40 {
            return Err(TodoError::DescriptionTooLong.into());
        }
        let user = &mut ctx.accounts.user;
        let todo = &mut ctx.accounts.todo;

        todo.todo_id = todo_count_index;
        todo.description = description;
        todo.status = TodoStatus::Todo;

        user.todo_count += 1;

        Ok(())
    }

    // Passer le statut du todo de "todo" à "done"
    pub fn update_todo(ctx: Context<UpdateTodo>, todo_count_index: u64) -> Result<()> {
        let todo = &mut ctx.accounts.todo;
        todo.status = TodoStatus::Done;
        Ok(())
    }

    pub fn delete_todo(ctx: Context<DeleteTodo>, todo_index: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        seeds = [b"user", signer.key.as_ref()],
        bump,
        payer = signer,
        space = 8 + 32 + 50 + 8 // allocation of space:
        // 8 bytes as default
        // 32 bytes for the public key
        // 50 bytes for the nickname
        // 8 bytes for the todo_count
    )]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(todo_count_index: u64)]
pub struct InitializeTodo<'info> {
    #[account(
        mut,
        seeds = [b"user", signer.key.as_ref()],
        bump
    )]
    pub user: Account<'info, User>,
    #[account(
        init,
        seeds = [b"todo", signer.key.as_ref(), &todo_count_index.to_le_bytes()],
        bump,
        payer = signer,
        space = 8 + 8 + 100 + 2
    )]
    pub todo: Account<'info, Todo>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(todo_count_index: u64)]
pub struct UpdateTodo<'info> {
    #[account(
        mut,
        seeds = [b"todo", signer.key.as_ref(), &todo_count_index.to_le_bytes()],
        bump,
    )]
    pub todo: Account<'info, Todo>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(todo_index: u64)]
pub struct DeleteTodo<'info> {
    #[account(
        mut,
        close = signer,
        seeds = [b"todo", signer.key.as_ref(), &todo_index.to_le_bytes()],
        bump,
    )]
    pub todo: Account<'info, Todo>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[account]
pub struct User {
    user_pubkey: Pubkey,
    nickname: String,
    todo_count: u64,
}

#[account]
pub struct Todo {
    todo_id: u64,
    status: TodoStatus,
    description: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum TodoStatus {
    Todo,
    Done,
}

#[error_code]
pub enum TodoError {
    #[msg("Description exceeds 40 characters")]
    DescriptionTooLong,
}
