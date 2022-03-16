pub mod config;
mod output;

use std::io::Write;
use std::{fs, env};
use termcolor::{WriteColor, BufferWriter, ColorChoice, ColorSpec, Color};

pub fn run(args: clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let flags = std::collections::HashMap::from([
        ('a', args.is_present("all")),
        ('r', args.is_present("reverse")),
        ('U', args.is_present("unsorted")),
        ('s', args.is_present("size")),
        ('h', args.is_present("human-readable")),
        ('b', args.is_present("base-1000")),
    ]);

    let bufwtr = BufferWriter::stdout(ColorChoice::Auto);
    let mut buffer = bufwtr.buffer();
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;

    if let Some(dirs) = args.values_of("DIRECTORY") {
        for dir in dirs {
            let pathbuf = fs::canonicalize(dir)?;
            writeln!(buffer, "➥ {}", pathbuf.to_str().unwrap())?;
            output::print_entries(&mut buffer, pathbuf.as_path(), &flags);
        }
    } else {
        let pathbuf = env::current_dir()?;
        output::print_entries(&mut buffer, pathbuf.as_path(), &flags);
    }
    bufwtr.print(&buffer)?;

    Ok(())
}