use async_recursion::async_recursion;
use clap::Parser;
use tokio::io::{
        AsyncBufReadExt,
        BufReader
    };
#[derive(Parser)]
struct Args {
    pattern: String,
}

// Avoid directories
const IGNORE_DIRS: [&str; 3] = ["/dev", "/proc", "/sys"];

async fn read_file(path: &String, pattern: &String) -> Result<(), Box<dyn std::error::Error>> {
    let f = tokio::fs::File::open(path).await?;

    match f.metadata().await {
        Ok(metadata) => {
            if metadata.is_dir() {
                return Ok(());
            }
        }
        Err(_e) => {},
    }

    let reader = BufReader::new(f);
    // get iterator over lines
    let mut lines = reader.lines();
    // hold lines in a vector
    let mut lines_vec = Vec::<String>::new();
    match lines.next_line().await {
        Ok(line) => {    
            match line {
                Some(line) => {
                    if line.contains(pattern) {
                        lines_vec.push(line);
                    }
                }
                None => {
                    return Ok(());
                },
            }
        }
        Err(_e) => {},
    }
    // Printout the filepath and all the lines found
    if lines_vec.len() > 0 {
        // print file path in blue
        println!("\x1b[34m{}\x1b[0m", path);
        for line in lines_vec {
            // Highlight the pattern found in the line red
            let line = line.replace(pattern, &format!("\x1b[31m{}\x1b[0m", pattern));
            println!("{}", line);
        }
    }

    Ok(())
}

#[async_recursion]
async fn app(path: &std::path::Path, pattern: &String) -> Result<(), Box<dyn std::error::Error>> {
    // Check if the file path exists if it does recurse and print it's contents
    // if it doesn't print an error message
    if path.exists() {
        // if path is hidden ignore
        if path.is_dir() {

         // check if it is an excluded directory
            if IGNORE_DIRS.contains(&path.to_str().unwrap()) {
                return Ok(());
            }

            let mut dir = tokio::fs::read_dir(path).await?;
            while let Some(res) = dir.next_entry().await? {
                let path = res.path();
                app(&path, &pattern).await?;
            }
        } else {
            // read file
            read_file(&path.to_str().unwrap().to_string(), pattern).await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let cwd = std::env::current_dir().unwrap();
    let path: &std::path::Path = std::path::Path::new(&cwd);
    let pattern = args.pattern;
    match app(path, &pattern).await {
        Ok(_) => println!("Done"),
        Err(_e) => {},
    }
}
