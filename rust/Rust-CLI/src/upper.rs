use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("error: input 2 words");
        return;
    }

    let input = &args[1];
    let upper = input.to_uppercase();

    println!("{}", upper);
}
