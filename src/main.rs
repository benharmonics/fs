fn main() {
    let args = fs::config::args();
    if let Err(e) = fs::run(args) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
