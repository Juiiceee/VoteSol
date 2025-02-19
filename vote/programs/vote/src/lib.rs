use anchor_lang::prelude::*;

declare_id!("2RCXjxrnEAJ4umjBydZet8YQNG1oJtKnmYkX8bm8fXZG");

#[program]
pub mod vote {
    use super::*;
    pub fn create_poll(ctx: Context<CreatePoll>, name: String, description: String) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        let id = &mut ctx.accounts.poll_id;
        if id.id == 0 {
            id.id = 0;
        }
        poll.poll_id = id.id;
        id.id += 1;
        poll.poll_name = name;
        poll.poll_description = description;
        poll.against = 0;
        poll.for_ = 0;
        poll.poll_start = 0;
        poll.poll_end = 0;
        poll.candidate_amount = 0;
        Ok(())
    }

    pub fn vote_poll(ctx: Context<VotePoll>, choose: bool) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        let voter = &mut ctx.accounts.signer;
        let candidate = &mut ctx.accounts.candidate;

        if (candidate.candidate_id == voter.key() && candidate.poll_id == poll.poll_id)
        {
            return Err(ErrorCode::AccountAlreadyVoted.into());
        }

        if choose {
            poll.for_ += 1;
        }
        else {
            poll.against += 1;
        }

        poll.candidate_amount += 1;

        candidate.candidate_id = voter.key();
        candidate.poll_id = poll.poll_id;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePoll<'info> {
    #[account(init, payer = signer, space = 8 + Poll::INIT_SPACE)]
    pub poll: Account<'info, Poll>,
    #[account(init_if_needed, payer = signer, space = 8 + PollId::INIT_SPACE)]
    pub poll_id: Account<'info, PollId>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClosePoll<'info> {
    #[account(mut, close = signer)]
    pub poll: Account<'info, Poll>,
    #[account(mut)]
    pub signer: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct VotePoll<'info> {
    #[account(mut)]
    pub poll: Account<'info, Poll>,
    #[account(init, payer = signer, space = 8 + Candidate::INIT_SPACE)]
    pub candidate: Account<'info, Candidate>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Poll {
    poll_id : u32,
    #[max_len(50)]
    poll_name: String,
    #[max_len(50)]
    poll_description: String,
    poll_start: u32,
    poll_end: u32,
    against : u16,
    for_: u16,
    candidate_amount: u32,
}

#[account]
#[derive(InitSpace)]
pub struct PollId {
    id: u32,
}

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    candidate_id: Pubkey,
    poll_id: u32,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Le compte AccountId n'a pas encore été initialisé.")]
    AccountIdNotInitialized,
    #[msg("Le candidat a deja vote ppour ce poll")]
    AccountAlreadyVoted,
}