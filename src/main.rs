#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub mincaml);

mod ast;

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

    let exp = mincaml::ExpParser::new().parse("1 +. 2 *. 3").unwrap();
    assert_eq!(&format!("{:?}", exp), "(1 +. (2 *. 3))");
}

#[test]
fn parse_functiondef() {
    assert!(mincaml::ExpParser::new()
        .parse(
            "
    let rec make_adder x =
        let rec adder y = x + y in
            adder in
                (make_adder 3) 7"
        )
        .is_ok());
    let assignment = mincaml::ExpParser::new()
        .parse("let x = 2 in x + 3")
        .unwrap();

    assert_eq!(
        &format!("{:?}", assignment),
        "((let [\"x\"] = 2) in ((x) + 3))"
    );

    let multiple_variables_assignment = mincaml::ExpParser::new()
        .parse("let (x, y) = 2 in x + 3")
        .unwrap();

    assert_eq!(
        &format!("{:?}", multiple_variables_assignment),
        "((let [\"x\", \"y\"] = 2) in ((x) + 3))"
    );

    let func_def = mincaml::ExpParser::new()
        .parse(
            "
    let rec make_adder x =
        let rec adder y = x + y in
            adder in
                (make_adder 3) 7",
        )
        .unwrap();

    assert_eq!(&format!("{:?}", func_def), "((\"make_adder\"  ([\"x\"]) return (((\"adder\"  ([\"y\"]) return (((x) + (y))) ) in (adder))) ) in (((make_adder) 3) 7))");
}

fn main() {}
