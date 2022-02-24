use std::{fs, io, error};
use std::collections::HashMap;
use std::path::Path;

pub struct Config {
    pub flags: Option<String>,
    pub dirs: Option<Vec<String>>
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let mut flags: Option<String> = None;
        let mut dirs: Option<Vec<String>> = None;

        // The default with no arguments is just to read the current directory
        if args.len() == 1 { return Ok(Config { flags, dirs }) }

        // Index of first directory argument (after the flags)
        let mut i0 = 1;

        // Implement using '?' to readdir with flags
        if args[1].chars().nth(0).unwrap() == '?' {
            flags = Some(String::from(&args[1]));
            i0 += 1;
        }

        // return the flags collected, and no dirs (since none were entered)
        if args.len() == 2 { return Ok(Config{ flags, dirs }) }

        let mut dirs_vec = Vec::new();
        for i in i0..args.len() {
            dirs_vec.push(String::from(&args[i]));
        }
        dirs = Some(dirs_vec);

        Ok(Config{ flags, dirs })
    }

    // Parse self.flags into commands to run (as a vector of &str)
    fn parse_flags(&self) -> Result<Vec<String>, &str> {
        if self.flags.is_none() { return Ok(vec![]) }   // return no commands if none were given

        let mut flags: Vec<char> = self.flags.as_ref()
            .unwrap_or(&"".to_string())
            .chars()
            .collect::<Vec<char>>();
        flags.remove(0);     // getting rid of '?' char

        if flags.is_empty() { return Ok(vec![]) }   // return no commands if none were given

        // All of our program's flags
        let all_flags = HashMap::from([
            ('a', "show_all_files"),
        ]);

        // list of commands to be run based on flags
        let mut commands = Vec::new();

        for c in &flags {
            if all_flags[c].is_empty() {
                return Err("Could not parse one or more flags. Use `rd -h` for help.")
            }
            commands.push(all_flags[c].to_string())
        }

        Ok(commands)
    }
}

fn _show_all_files() {}

fn read_directory(path: &Path, flags: &[String]) -> Result<(), Box<dyn error::Error>> {
    let mut entries = fs::read_dir(path)?;

    let mut filtered_entries = entries.map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<std::path::PathBuf>, io::Error>>()?;

    filtered_entries.sort();

    for entry in filtered_entries {
        println!("{}", entry.as_os_str().to_str().unwrap());
    }

    Ok(())
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let commands: Vec<String> = config.parse_flags()?;
    let current_dir = std::env::current_dir()?;
    let current_path = current_dir.as_path();

    // Our edge case is that `dirs` is empty
    if config.dirs.is_none() {
        read_directory(current_path, &commands)?;
        return Ok(())
    }

    for dir in config.dirs.as_ref().unwrap() {
        // fs::canonicalize gives us the full path, rather than a truncated path.
        read_directory(fs::canonicalize(dir).unwrap().as_path(), &commands)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn parse_flags1() {
        let s = parse_flags("&allen").unwrap();
        assert_eq!(vec!["show_all_files"], s);
    }

    #[test]
    fn parse_flags2() {
        let s = parse_flags("&a").unwrap();
        assert_eq!(vec!["show_all_files"], s);
    }
}