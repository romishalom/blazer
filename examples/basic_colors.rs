use blazer::Stylize;

fn main() {
    println!("{}", "Standard colors:".bold());
    println!("{}", "Red".red());
    println!("{}", "Green".green());
    println!("{}", "Blue".blue());
    println!("{}", "Yellow on blue".yellow().on_blue());

    println!("\n{}", "Styles:".bold());
    println!("{}", "Bold".bold());
    println!("{}", "Italic".italic());
    println!("{}", "Underline".underline());
    println!("{}", "Strikethrough".strikethrough());
    println!("{}", "Dim".dim());

    println!("\n{}", "Advanced colors:".bold());
    println!("{}", "256-color".fixed(202));
    println!("{}", "RGB truecolor".rgb(64, 224, 208).bold());
}