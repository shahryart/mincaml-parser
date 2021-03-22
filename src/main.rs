extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "mincaml.pest"]
pub struct MinCamlParser;

fn main() {
    let successful_parse = MinCamlParser::parse(Rule::lparen, "(");
    println!("{:?}", successful_parse);
}
