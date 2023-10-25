use std::fs;
use std::path::Path;
use anyhow::Result;
use csv::Reader;
use indicatif::ProgressBar;
use rayon::prelude::*;
use reqwest::blocking::ClientBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Message {
    #[serde(rename = "ID")]
    id: u64,
    #[serde(rename = "Timestamp")]
    timestamp: String,
    #[serde(rename = "Contents")]
    contents: String,
    #[serde(rename = "Attachments")]
    attachments: String
}

fn main() -> Result<()> {
    let client = ClientBuilder::new().build()?;
    let out = Path::new("out");
    if !out.exists() {
        fs::create_dir(out)?;
    }

    let files: Vec<_> = glob::glob("**/*.csv")?
        .filter_map(|x| x.ok())
        .collect();

    let bar = ProgressBar::new(files.len() as u64);
    bar.set_message("downloading attachments");

    // god awful code, so goddamn ugly, please don't look at this
    // https://www.reddit.com/r/ProgrammerHumor/comments/27yykv/indent_hadouken

    files.into_par_iter().try_for_each(|path| -> Result<()> {
        let mut reader = Reader::from_path(path)?;
        for result in reader.deserialize() {
            let message: Message = result?;
            for attachment in message.attachments.split_whitespace() {
                if let Ok(body) = client.get(attachment).send() {
                    let full_path = body.url().path();
                    let path = match full_path.strip_prefix('/') {
                        Some(path) => path,
                        None => full_path
                    }.replace('/', "_");

                    let out = out.join(path);
                    if let Ok(bytes) = body.bytes() {
                        fs::write(out, bytes)?;
                    }
                }
            }
        }
        bar.inc(1);
        Ok(())
    })?;

    bar.finish();

    Ok(())
}
