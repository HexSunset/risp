pub use pest::Parser;
pub use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "math.pest"]
pub struct MathParser;

pub use pest::iterators::Pair;
pub fn tree_depth(p: Pair<Rule>) -> usize {
    let mut max_depth = 0;
    let children: Vec<Pair<Rule>> = p.into_inner().collect();

    if children.len() == 0 {
	return 0;
    }

    for c in children {
	use std::cmp::max;
	max_depth = max(max_depth, 1 + tree_depth(c));
    }
    
    return max_depth;
}
