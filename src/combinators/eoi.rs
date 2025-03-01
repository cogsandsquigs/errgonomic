use crate::parser::{errors::Result, input::Underlying, state::State};

/// Parses an end of input.
pub fn eoi<I: Underlying>(state: State<I>) -> Result<I, ()> {
    if state.input.is_empty() {
        Ok((state, ()))
    } else {
        todo!("Need to error out here!")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_parse_eoi() {
        let result: (crate::parser::state::State<&str>, ()) = super::eoi("".into()).unwrap();
        assert!(!result.0.errors().any_errs());
        assert_eq!(result.0.errors().num_errors(), 0);
        assert_eq!(result.0.errors().errors().len(), 0);
        assert_eq!(result.0.input, "");

        todo!("Need to test error cases!");
    }
}
