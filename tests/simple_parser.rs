use errgonomic::{
    combinators::*,
    parser::{errors::Result, state::State, Parser},
};

fn the_parser(state: State<&str>) -> Result<&str, (String, &str)> {
    many_until(is("test"), is("done"))
        .map(|(manys, _)| {
            manys
                .iter()
                .map(|many| many.as_inner())
                .collect::<Vec<_>>()
                .join(", ")
        })
        .then(is("hello, world!").map(|o| o.as_inner()))
        .process(state)
}

#[test]
fn simple_parser() {
    let result = the_parser.parse("testtesttestdonehello, world!").unwrap();
    assert_eq!(result, ("test, test, test".into(), "hello, world!"));
}
