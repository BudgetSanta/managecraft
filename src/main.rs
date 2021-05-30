use std::process;

fn main() {
    if let Err(e) = mc_mng::run() {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
