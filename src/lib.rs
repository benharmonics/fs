use clap::{arg, Command, ArgMatches};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::{fs, io, error};
use std::io::Write;
use std::path::PathBuf;
use std::collections::HashMap;

/* CLI argument parsing via clap crate */
pub fn args() -> ArgMatches {
    Command::new("readdir")
        .version("1.0")
        .author("benharmonics")
        .about("Reads items in a given directory")
        .arg(arg!(-a --all "Show hidden files"))
        .arg(arg!(-v --verbose "Show more info"))
        .arg(arg!([DIRECTORY] ... "One or more directories to read"))
        .get_matches()
}

/* Reads the directory contents and prints them to stdout */
fn write_to_stdout(stdout: &mut StandardStream, buf: PathBuf, flags: &HashMap<char, bool>)
                   -> Result<(), Box<dyn error::Error>> {
    let mut entries: Vec<PathBuf> = fs::read_dir(buf.as_path())
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<PathBuf>, io::Error>>()
        .unwrap_or(vec![]);
    entries.sort();

    let filenames: Vec<&str> = entries.iter()
        .map(|p| p.file_name().unwrap())
        .map(|s| s.to_str().unwrap())
        .collect();

    for i in 0..filenames.len() {
        if !flags[&'a'] && filenames[i].starts_with('.') { continue; }
        if entries[i].is_dir() {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
        } else {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        }
        writeln!(&mut *stdout, "{}", filenames[i])?;
    }

    Ok(())
}

/* Function is called from main.rs; program exits with an error if anything fails. */
pub fn run(args: clap::ArgMatches) -> Result<(), Box<dyn error::Error>> {
    // flags parsed from arguments, normal CLI stuff
    let flags = HashMap::from([
        ('a', args.is_present("all")),
        ('v', args.is_present("verbose")),
    ]);

    // Set up stdout stream (as opposed to a buffer)
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // If user entered no optional paths to be read, just read the current directory.
    let dirs: Option<_> = args.values_of("DIRECTORY");
    if dirs.is_none() {
        let current_dir = std::env::current_dir()?;
        write_to_stdout(&mut stdout, current_dir, &flags)?;
    } else {
        for dir in dirs.unwrap().collect::<Vec<_>>() {
            let dir_path = fs::canonicalize(dir)?;
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;     // It can change
            writeln!(&mut stdout, " ==> {} <== ", dir_path.as_os_str().to_str().unwrap())?;
            write_to_stdout(&mut stdout, dir_path, &flags)?;
        }
    }

    Ok(())
}
