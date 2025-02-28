use crate::parsers::{
    input::{PInput, Underlying},
    result::PResult,
};

pub fn id<I: Underlying, E>(input: PInput<I>) -> PResult<I, PInput<I>, E> {
    Ok((input.take(input.len()), input))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parsers::Parser;

    #[test]
    fn can_parse_id() {
        let input = "hello".into();
        assert_eq!(
            id::<&str, ()>.parse(input),
            Ok((PInput::new_at("hello", 5), PInput::new_at("hello", 0)))
        );
    }
}
