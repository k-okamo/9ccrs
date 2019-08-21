use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprint!("Usage: 9ccrs <code>\n");
        process::exit(1);
    }

    print!(".intel_syntax noprefix\n");
    print!(".global main\n");
    print!("main:\n");
    print!("\tmov rax, {}\n", &args[1]);
    print!("\tret\n");
}
