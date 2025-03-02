use errgonomic::{combinators::*, parser::Parser};

#[test]
fn test() {
    let mut parser = many_until(is("test"), is("done"))
        .map(|(manys, _)| {
            manys
                .iter()
                .map(|many| many.as_inner())
                .collect::<Vec<_>>()
                .join(", ")
        })
        .chain(is("hello, world!").map(|o| o.as_inner()));

    let result = parser.parse("testtesttestdonehello, world!").unwrap();
    assert_eq!(result, ("test, test, test".into(), "hello, world!"));
}
