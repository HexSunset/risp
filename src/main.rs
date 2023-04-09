use anyhow::Result;

fn main() -> Result<()> {
    repl()?;

    Ok(())
}

fn repl() -> Result<()> {
    println!("RISP v0.0.0");
    println!("Exit at any time with Ctrl+c");
    println!("");

    let mut rl = rustyline::DefaultEditor::new()?;
    
    loop {
	let line = rl.readline("risp> ")?;
	rl.add_history_entry(&line)?;

	use risp::parse::*;
	use risp::eval::*;
	let mut tree = match MathParser::parse(Rule::Expression, &line) {
	    Ok(t) => t,
	    Err(e) => {
		eprintln!("{e}");
		continue;
	    },
	};

	println!("{}", eval_math(tree.next().unwrap()));
    }
}
