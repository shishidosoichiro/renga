fn main() {
    if let Err(e) = fbim::run() {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}
