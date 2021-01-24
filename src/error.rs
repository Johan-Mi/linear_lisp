use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("tried to call swap_with_car() on an Atom")]
    SwapWithCarAtom,
    #[error("tried to call swap_with_cdr() on an Atom")]
    SwapWithCdrAtom,
    #[error("tried to call pop() with a non-Nil value as destination")]
    PopIntoNonNil,
    #[error("tried to call pop() with an Atom as source")]
    PopFromAtom,
}
