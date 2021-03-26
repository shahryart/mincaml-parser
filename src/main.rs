#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub mincaml);

#[test]
fn parse_expressions() {
    assert!(mincaml::ExpParser::new().parse("true").is_ok());
    assert!(mincaml::ExpParser::new().parse("false").is_ok());
    assert!(mincaml::ExpParser::new().parse("13.23").is_ok());
    assert!(mincaml::ExpParser::new().parse("13.23 + 2").is_ok());
    assert!(mincaml::ExpParser::new()
        .parse("this_might_be_an_identifier")
        .is_ok());

    assert!(mincaml::ExpParser::new().parse("13.23 + 2").is_ok());
    assert!(mincaml::ExpParser::new()
        .parse("if (true) then 2 else 3")
        .is_ok());
    assert!(mincaml::ExpParser::new()
        .parse("let x = 2 in x + 2")
        .is_ok());
    assert!(mincaml::ExpParser::new()
        .parse("let rec foo y = foo y + 2 in foo 2")
        .is_ok());
}

fn main() {}
