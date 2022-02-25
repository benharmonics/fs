fn main() {
    let args = readdir::args();
    if let Err(e) = readdir::run(args) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
