use clap::{Command, Arg, ArgAction};

fn main() {
    // Set a breakpoint here to inspect the command setup
    let command_line_arguments = Command::new("echor")
        .version("0.1.0")
        .author("Nazli <nazliander1@gmail.com>")
        .about("Prints the given arguments to the console")
        .arg(Arg::new("text").help("The text to print").required(true).num_args(1..))
        .arg(Arg::new("omit_newline").help("Do not print the trailing newline").short('n').action(ArgAction::SetTrue))
        .get_matches();

    // Set breakpoints here to inspect parsed values
    let text_values: Vec<String> = command_line_arguments.get_many("text").unwrap().cloned().collect();
    let omit_newline = command_line_arguments.get_flag("omit_newline");
    
    let ending = if omit_newline { "" } else { "\n" };
    // Join the text values
    let joined_text = text_values.join(" ");

    print!("{joined_text}{ending}");

}
