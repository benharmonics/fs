use clap::{arg, Arg, ColorChoice, Command, ArgMatches};

pub fn args() -> ArgMatches {
    let clap_color_choice = if std::env::var_os("NO_COLOR").is_none() {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    };

    Command::new("fs")
        .version("0.1.2")
        .author("benharmonics")
        .about("Reads names of items in a directory and prints them to console (like ls).")
        .color(clap_color_choice)
        .arg(
            Arg::new("all")
                .long("all")
                .short('a')
                .help("Show hidden files"),
        )
        .arg(
            Arg::new("unsorted")
                .long("unsorted")
                .short('u')
                .help("Don't sort items (use directory ordering)"),
        )
        .arg(
            Arg::new("case-sensitive")
                .long("case-sensitive")
                .short('c')
                .help("Case-sensitive sorting")
        )
        .arg(
            Arg::new("reverse")
                .long("reverse")
                .short('r')
                .help("Reverse output order"),
        )
        .arg(
            Arg::new("size")
                .long("size")
                .short('s')
                .help("Get files size (in bytes)"),
        )
        .arg(
            Arg::new("base-1000")
                .long("base-1000")
                .short('b')
                .help("With '-s', display filesize such that 1kB=1000B")
        )
        .arg(
            Arg::new("human-readable")
                .long("human-readable")
                .short('h')
                .help("With '-s', gives file size in kB, MB, etc")
        )
        .arg(arg!([DIRECTORY] ... "One or more directories to read"))
        .get_matches()
}
