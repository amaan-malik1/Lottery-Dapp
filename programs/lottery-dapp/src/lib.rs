mod constants;
mod error;
use crate::{constants::*, error::*};

use anchor_lang::{
    prelude::*,
    solana_program::{
        hash::hash,
        program::invoke,
        system_instruction::transfer,
    }
};

declare_id!("A91MerwtrgEycr8cuKwpxw7Z81QbwMvGK9iRL76LXpDS");

//lottery program/contract
#[program]
pub mod lottery_contract{
    use super::*;

    //initialize a master
    pub fn init_master(ctx:Context<InitMaster>) -> Result<()> {
        Ok(())
    }

    //create lottery function
    pub fn create_lottery(ctx:Context<CreateLottery>, set_ticket_price:u64) -> Result<()>{
        let lottery =&mut ctx.accounts.lottery;
        let master =&mut ctx.accounts.master;

        //increment the last_ticket_id 
        master.last_id += 1;

        //lottery id
        lottery.id = master.last_id;

        //get authority
        lottery.authority = ctx.accounts.authority.key();

        //set ticket price
        lottery.ticket_price = set_ticket_price;

        //last ticket id
        // lottery.last_ticket_id = master;

        msg!("Created lottery: {}", lottery.id);
        msg!("Authority: {}", lottery.authority);
        msg!("Ticket price: {}", lottery.ticket_price);

        Ok(())
    }

    //buy ticket
    pub fn buy_ticket(ctx:Context<BuyTicket>, lottery_id:u32) -> Result<()>{
        let lottery = &mut ctx.accounts.lottery;
        let ticket = &mut ctx.accounts.ticket;
        let buyer = &mut ctx.accounts.buyer;

        //if checks
        if lottery.winner_id.is_some() {
            return err!(LotteryError::WinnerAlreadyExisted);           
        }


        //send SOL to the lotery account
        invoke(
            &transfer(
                &buyer.key(),
                &lottery.key(),
                lottery.ticket_price,
            ),
           &[
                buyer.to_account_info(),
                lottery.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
           ],
        )?;

        lottery.last_ticket_id += 1;

        ticket.id = lottery.last_ticket_id;
        ticket.lottery_id = lottery_id;
        ticket.authority  = buyer.key();

        msg!("Bought ticket id: {}", ticket.id);
        msg!("Ticket authority id: {}", ticket.authority);


        Ok(())
    }

    //pick winner
   pub fn pick_winner(ctx:Context<PickWinner>, lottery_id: u32) -> Result<()>{
        let lottery = &mut ctx.accounts.lottery;
        
        let clock = Clock::get()?;

        // Pick a psuedo-random winner
        let pseudo_random_number = ((u64::from_le_bytes(
            <[u8; 8]>::try_from(&hash(&clock.unix_timestamp.to_be_bytes()).to_bytes()[..8])
                .unwrap(),
        )) * clock.slot
            % u32::MAX as u64) as u32;

        require!(lottery.last_ticket_id > 0, LotteryError::NoTickets);
        let winner_id = (pseudo_random_number % lottery.last_ticket_id) + 1;
        lottery.winner_id = Some(winner_id);

        msg!("Winner id:{}", winner_id);

        Ok(())
        
    }

    pub fn claim_prize(ctx:Context<ClaimPrize>, _lottery_id:u32, _ticket_id:u32) -> Result<()>{
        let lottery =&mut ctx.accounts.lottery;
        let ticket = &mut ctx.accounts.ticket;
        let winner = &mut ctx.accounts.authority;

        if lottery.claimed {
            return err!(LotteryError::AlreadyClaimed);        
        }

        match lottery.winner_id {
            Some(winner_id) => {
                if winner_id != ticket.id {
                    return err!(LotteryError::InvalidWinner)
                }
            }

            None => return err!(LotteryError::WinnerNotChosen)
        }

        //transfer the lottery prize from lottery PDA to winner
        let total_prize = lottery.ticket_price.checked_mul(lottery.last_ticket_id.into()).unwrap();

        **lottery.to_account_info().try_borrow_mut_lamports()? -= total_prize;  //reduce the prize amount from PDA 
        **winner.to_account_info().try_borrow_mut_lamports()? += total_prize;   //add the amount to the winner 

        lottery.claimed = true;

        msg!("Winner: {} claimed {} money from the lottery id: {} whose ticket id: {}",
            winner.key(),
            total_prize,
            lottery.id, 
            ticket.id,
        );

        Ok(())
    }

}

#[derive(Accounts)]
pub struct InitMaster<'info>{
    #[account(
        init,
        payer = payer,
        space = 8 + 4, 
        seeds = [MASTER_SEED.as_bytes()],
        bump,
    )]
    pub master:Account<'info, Master>,

    #[account(mut)]
    pub payer:Signer<'info>,

    pub system_program:Program<'info, System>,
}

#[account]
pub struct Master{
    pub last_id:u32,
}

#[derive(Accounts)]
pub struct CreateLottery<'info>{
    #[account(
        init,
        payer = authority,
        space = 4 + 32 + 8 + 4  + 4 + 1 + 8,
        seeds = [LOTTERY_SEED.as_bytes(), &(master.last_id + 1).to_le_bytes()], //here master.lastId is the var above and here it's a one more seed to make the PDA unique
        bump,
    )]
    pub lottery:Account<'info, Lottery>,    //type of account is Lottery

    //accessing the master
    #[account(
        mut,
        seeds = [MASTER_SEED.as_bytes()],
        bump,
    )]
    pub master: Account<'info, Master>,

    #[account(mut)]
    pub authority:Signer<'info>,

    pub system_program:Program<'info, System>
}

#[account]
pub struct Lottery{
    pub id:u32,
    pub authority:Pubkey,
    pub ticket_price:u64,
    pub last_ticket_id:u32,
    pub winner_id:Option<u32>,
    pub claimed:bool,
}

#[derive(Accounts)]
#[instruction(lottery_id:u32)]
pub struct BuyTicket<'info>{
    #[account(
        mut,
        seeds = [LOTTERY_SEED.as_bytes(), &lottery_id.to_le_bytes()],
        bump,
    )]
    pub lottery:Account<'info, Lottery>,

    #[account(
        init,
        payer = buyer,
        space = 4 + 4 + 32 + 8,
        seeds = [
            TICKET_SEED.as_bytes(),
            lottery.key().as_ref(),
            &(lottery.last_ticket_id + 1).to_le_bytes(),
        ],
        bump,
    )]
    pub ticket:Account<'info, Ticket>,

    #[account(mut)]
    pub buyer:Signer<'info>,

    pub system_program:Program<'info, System>,
}

#[account]
pub struct Ticket{
    pub id:u32,
    pub authority:Pubkey,
    pub lottery_id:u32,
}

#[derive(Accounts)]
#[instruction(lottery_id: u32)]
pub struct PickWinner<'info>{
    #[account(
        mut,
        seeds = [LOTTERY_SEED.as_bytes(), &lottery_id.to_le_bytes()],
        bump,
        has_one = authority,
    )]
    pub lottery: Account<'info, Lottery>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(lottery_id: u32, ticket_id: u32)]
pub struct ClaimPrize<'info>{
    #[account(
        mut,
        seeds = [LOTTERY_SEED.as_bytes(), &lottery_id.to_le_bytes()],
        bump,
    )]
    pub lottery:Account<'info, Lottery>,

    #[account(
        seeds = [
            TICKET_SEED.as_bytes(),
            lottery.key().as_ref(),
            &ticket.id.to_le_bytes(),
        ],
        bump,
        has_one = authority,
    )]
    pub ticket:Account<'info, Ticket>,

    #[account(mut)]
    pub authority:Signer<'info>,

    pub system_program:Program<'info, System>

}