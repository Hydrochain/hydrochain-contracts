use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Producer cannot buy his own containers")]
    ProducerCannotBuy {},

    #[error("Forbidden - Sender does not own that container")]
    ForbiddenLackOfOwnership {},

    #[error("Forbidden - container does not have Created status")]
    ForbiddenStatusNotCreated {},

    #[error("Forbidden - container does not have Shipped status")]
    ForbiddenStatusNotShipped {},

    #[error("Denom of sent tokens does not match price")]
    BuyDenomDoesntMatchPrice {},

    #[error("Not enough token sent to buy container")]
    NotEnoughTokensSent {},

    #[error("Transaction failure - no tokens sent")]
    NoTokensSent {},
}
