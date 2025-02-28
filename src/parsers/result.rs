use super::input::PInput;

pub type PResult<I, O, E> = Result<(PInput<I>, O), Vec<E>>;
