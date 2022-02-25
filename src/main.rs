fn main() {
    let matches = readdir::get_matches();
    if let Err(e) = readdir::run(matches) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
