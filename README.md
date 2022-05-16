# fs
**fs** reads the contents of a directory and prints them to the console (like `ls`).

"fs" for "files" (...?) and because it's easy to type! ("rd" for "readdir" I thought would've been better but it's already "remove dir", so...)

Only a few flags have been implemented so far, but it's what you'd expect from `ls` (i.e. '-s', '-h', '-a', etc).

Run the program with the '--help' flag to get more information.

## Installation

### Cargo
Clone the repository, then once you've navigated into the project directory:
```bash
cargo install
```

By default, Cargo installs new binaries to `$HOME/.cargo/bin`.
