use anchor_lang::prelude::error_code;

#[error_code]
pub enum LotteryError {
    #[msg("Winner already existed")]
    WinnerAlreadyExisted,

    #[msg("Can't choose a winner when there's no ticket")]
    NoTickets,    
    
    #[msg("Winner has not been chosen")]
    WinnerNotChosen,

    #[msg("Can't choose a winner when there's no ticket")]
    InvalidWinner,

    #[msg("The Prize has already been claimed")]
    AlreadyClaimed,
}