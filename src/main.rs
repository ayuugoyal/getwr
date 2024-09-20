use clap::{App, Arg};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("getwr")
        .version("0.1.0")
        .author("ayugoyal")
        .about("wget clone in rust")
        .arg(
            Arg::with_name("URL")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("url to download"),
        )
        .get_matches();

    let url = matches.value_of("URL").unwrap();
    println!("URL to download: {}", url);

    download(url, false)?;

    Ok(())
}

// Function to create a progress bar using indicatif
fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    let bar = match quiet_mode {
        true => ProgressBar::hidden(),
        false => match length {
            Some(len) => ProgressBar::new(len),
            None => ProgressBar::new_spinner(),
        },
    };

    bar.set_message(msg.to_owned());
    if let Some(_) = length {
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}")
                .unwrap()
                .progress_chars("-#-> "),
        );
    } else {
        bar.set_style(ProgressStyle::default_spinner());
    }

    bar
}

// Function to download the file using reqwest
fn download(target: &str, quiet_mode: bool) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let mut resp = client.get(target).send()?;

    println!("HTTP request sent... {}", resp.status());

    if resp.status().is_success() {
        let headers = resp.headers();
        let ct_len = headers
            .get("content-length")
            .and_then(|val| val.to_str().ok())
            .and_then(|val| val.parse::<u64>().ok());

        let ct_type = headers.get("content-type").unwrap();

        let ct_type_str = ct_type.to_str().unwrap_or("unknown");

        println!("Type: {}", ct_type_str);

        match ct_len {
            Some(len) => {
                println!("Length: {} ({} bytes)", len, len);
            }
            None => {
                println!("Length: unknown");
            }
        }

        println!("Type: {}", ct_type.to_str().unwrap());

        let fname = target.split('/').last().unwrap_or("output.bin");

        println!("Saving to: {}", fname);

        let chunk_size = ct_len.map_or(1024usize, |x| x as usize / 99);

        let mut buf = Vec::new();
        let bar = create_progress_bar(quiet_mode, fname, ct_len);

        loop {
            let mut buffer = vec![0; chunk_size];
            let bcount = resp.read(&mut buffer[..])?;
            if bcount == 0 {
                break;
            }
            buffer.truncate(bcount);
            buf.extend_from_slice(&buffer);
            bar.inc(bcount as u64);
        }

        bar.finish();
        save_to_file(&buf, fname)?;
    } else {
        println!("Error: Failed to download");
    }

    Ok(())
}

fn save_to_file(buf: &[u8], fname: &str) -> Result<(), io::Error> {
    let check = File::open(fname);
    if check.is_ok() {
        println!("File already exists. Overwrite / save again / exit? (o/s/e)");
        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        if response.trim() != "o" && response.trim() != "s" {
            println!("Exiting...");
            return Ok(());
        } else if response.trim() == "s" {
            println!("Enter new filename: ");
            let mut new_fname = String::new();
            io::stdin().read_line(&mut new_fname).unwrap();
            fname.split(".").next().unwrap();
            new_fname = format!("{}.{}", new_fname.trim(), fname.split(".").last().unwrap());
            return save_to_file(buf, new_fname.trim());
        }
    }
    let mut file = File::create(fname)?;
    file.write_all(buf)?;
    Ok(())
}
