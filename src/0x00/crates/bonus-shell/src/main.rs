use std::{
    env::{self, set_current_dir},
    error::Error,
    fs, io::{self, Write},
    path::Path,
    process,
};
use whoami;

type ShellResult<T> = std::result::Result<T, Box<dyn Error>>;

fn pwd() -> ShellResult<()> {
    let path = env::current_dir()?;
    print!("{}", path.display());
    Ok(())
}

fn cd<P: AsRef<Path>>(path: P) -> ShellResult<()> {
    set_current_dir(path)?;
    pwd()?;
    println!("");
    Ok(())
}

fn ls(dir: &Path, cb: &dyn Fn(&Path)) -> ShellResult<()> {
    if dir.is_dir() {
        let mut entries = fs::read_dir(dir)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        entries.sort();

        for entry in entries {
            cb(&entry.as_path())
        }
    }
    println!("");

    Ok(())
}

fn call_back(path: &Path) {
    if let Some(name) = path.file_name() {
        print!("{}  ", name.to_string_lossy())
    }
}

fn exit() -> ShellResult<()> {
    println!("exit!");
    process::exit(0);
}

fn print_prompt() {
    print!("{}@{}", whoami::username().unwrap_or_else(|_| "<unknown>".to_string()), 
                    whoami::hostname().unwrap_or_else(|_| "<unknown>".to_string()));
    let _ = pwd();
    print!("$ ");
    let _ = io::stdout().flush();
}

fn main() -> ShellResult<()> {
    let mut input = String::new();

    loop {
        print_prompt();
        input.clear();
        io::stdin().read_line(&mut input)?;
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() { continue; }

        let command = parts[0];
        let args = &parts[1..];
        match command {
            "pwd" =>  {
                pwd()?;
                println!("");
            },
            "exit" => { exit()?; }
            "cd" => { 
                let target = args.get(0).unwrap_or(&"."); 
                if let Err(_) = cd(target) {
                    println!("cd: no such file or directory: {}", target);
                }
            },
            "ls" => { 
                let target = args.get(0).unwrap_or(&".");
                if let Err(_) = ls(Path::new(target), &call_back) {
                    println!("ls: cannot access '{}': no such file or directory", target);
                }
            },
            _ => println!("lshell: command not found {}", command),
        }
    }
}
