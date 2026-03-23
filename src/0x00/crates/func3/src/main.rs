use console::{Alignment, Term, pad_str, style};

fn main() {
    colored_output();
}

pub fn colored_output() {
    println!("{}: Hello World!", style("INFO").green());
    println!(
        "{}: {}",
        style("WARNING").yellow().bold().underlined(),
        style("I'm a teapot").yellow()
    );

    let term = Term::stdout();
    let (_, width) = term.size();
    let styled_msg = style("ERROR: KERNEL PANIC!!!").red().bold().to_string();
    let output = pad_str(&styled_msg, width as usize, Alignment::Center, None);
    println!("{}", output);
}
