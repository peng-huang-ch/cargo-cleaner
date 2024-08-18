use cargo_cleaner::cli;

fn main() {
    if let Err(err) = cli::run() {
        println!("Error: {err:?}");
        std::process::exit(1);
    }
    println!("Done");
}
