fn main() {
    if let Err(e) = renga::run() {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}
