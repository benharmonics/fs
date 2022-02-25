use clap::{arg, Command, ArgMatches};
use std::{fs, io, error};
use std::path::PathBuf;
use std::collections::HashMap;

pub fn get_matches() -> ArgMatches {
    Command::new("readdir")
        .version("1.0")
        .author("benharmonics")
        .about("Reads items in a given directory")
        .arg(arg!(-a --all "Show hidden files"))
        .arg(arg!(-v --verbose "Show more info"))
        .arg(arg!([DIRECTORY] ... "One or more directories to read"))
        .get_matches()
}

fn read_contents(p: PathBuf, flags: &HashMap<char, bool>) {
    let mut entries = fs::read_dir(p.as_path())
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<PathBuf>, io::Error>>()
        .unwrap_or(vec![]);
    entries.sort();

    let filenames: Vec<&str> = entries.iter()
        .map(|p| p.file_name().unwrap())
        .map(|s| s.to_str().unwrap())
        .collect();

    for filename in filenames{
        if !flags[&'a'] && filename.starts_with('.') { continue; }
        println!("{}", filename);
    }
}

pub fn run(args: clap::ArgMatches) -> Result<(), Box<dyn error::Error>> {
    let flags = HashMap::from([
        ('a', args.is_present("all")),
        ('v', args.is_present("verbose")),
    ]);

    let dirs: Option<_> = args.values_of("DIRECTORY");
    if dirs.is_none() {
        let current_dir = std::env::current_dir()?;
        read_contents(current_dir, &flags);
    } else {
        for dir in dirs.unwrap().collect::<Vec<_>>() {
            let dir_path = fs::canonicalize(dir)?;
            println!(" ==> {} <== ", dir_path.as_os_str().to_str().unwrap());
            read_contents(dir_path, &flags);
        }
    }

    Ok(())
}
