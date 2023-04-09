use crate::parse::*;

// Use of unwrap is justified because the iterator is supposed to contain these items
// unless something is wrong with the parsing library. We have bigger problems in that case.
pub fn eval_math(pair: Pair<Rule>) -> i64 {
    if pair.as_rule() == Rule::Number {
	return pair.as_str().parse::<i64>().unwrap()
    }

    if pair.as_rule() == Rule::Expression {
	let mut children = pair.into_inner();
	let _open_paren = children.next();
	
	let operator = children.next().unwrap().as_str();
	// TODO: we're currently allowing multiple operands to parse, but not handling them.
	let operand_a = eval_math(children.next().unwrap());
	let operand_b = eval_math(children.next().unwrap());
	
	let _close_paren = children.next();
	return compute(operator, operand_a, operand_b);
    }

    unreachable!()
}

fn compute(p: &str, a: i64, b: i64) -> i64 {
    match p {
	"+" => a + b,
	"-" => a - b,
	"*" => a * b,
	"/" => a / b,
	_ => unreachable!(),
    }
}
