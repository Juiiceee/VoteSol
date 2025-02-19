use anchor_lang::prelude::*;

declare_id!("J7GcJ4qmKtj1KD8N56pPUtDuV3Njm5DEXhe92tXhbKzM");

#[program]
mod vote {
    use super::*;

    // Crée un nouveau poll avec un PDA comme adresse
    pub fn create_poll(ctx: Context<CreatePoll>, name: String, description: String) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.poll_name = name.clone(); // Stocke le nom du poll
        poll.poll_description = description;
        poll.against = 0;
        poll.for_ = 0;
        poll.candidate_amount = 0;
        Ok(())
    }

    // Vote pour un poll
    pub fn vote_poll(ctx: Context<VotePoll>, choose: bool) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        let voter = &mut ctx.accounts.signer;
        let candidate = &mut ctx.accounts.candidate;

        // Vérifie si le votant a déjà voté
        if candidate.candidate_id == *voter.key && candidate.poll_id == poll.poll_id {
            return Err(ErrorCode::AccountAlreadyVoted.into());
        }

        // Met à jour les votes
        if choose {
            poll.for_ += 1;
        } else {
            poll.against += 1;
        }

        poll.candidate_amount += 1;

        // Enregistre le vote
        candidate.candidate_id = *voter.key;
        candidate.poll_id = poll.poll_id;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String)] // Reçoit `name` comme argument supplémentaire
pub struct CreatePoll<'info> {
    // PDA pour le poll
    #[account(
        init,
        payer = signer,
        space = 8 + Poll::INIT_SPACE,
        seeds = [b"poll", name.as_bytes()], // Utilise `name` comme seed
        bump
    )]
    pub poll: Account<'info, Poll>,

    // Signer qui paie pour la création du compte
    #[account(mut)]
    pub signer: Signer<'info>,

    // Programme système
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VotePoll<'info> {
    // Compte poll
    #[account(mut)]
    pub poll: Account<'info, Poll>,

    // Compte candidat (pour enregistrer le vote)
    #[account(
        init,
        payer = signer,
        space = 8 + Candidate::INIT_SPACE,
        seeds = [b"candidate", poll.key().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub candidate: Account<'info, Candidate>,

    // Signer qui paie pour la création du compte
    #[account(mut)]
    pub signer: Signer<'info>,

    // Programme système
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Poll {
    poll_id: u32, // Identifiant unique du poll
    #[max_len(50)]
    poll_name: String, // Nom du poll
    #[max_len(50)]
    poll_description: String, // Description du poll
    against: u16, // Nombre de votes contre
    for_: u16, // Nombre de votes pour
    candidate_amount: u32, // Nombre total de candidats
}

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    candidate_id: Pubkey, // Clé publique du votant
    poll_id: u32, // Identifiant du poll
}

#[error_code]
pub enum ErrorCode {
    #[msg("Le candidat a déjà voté pour ce poll.")]
    AccountAlreadyVoted,
}