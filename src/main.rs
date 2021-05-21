use async_std::fs::File;
use async_std::io::{stdin, BufReader};
use async_std::prelude::*;
use clap::Clap;
use std::error::Error;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Collins Huff chuff@paloaltonetworks.com")]
struct Opts {
    /// Path to input file, defaults to stdin
    #[clap(short, long)]
    in_file: Option<String>,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();
    env_logger::init();
    match &opts.in_file {
        Some(f) => {
            let f = File::open(&f).await?;
            let b = BufReader::new(f);
            scrape_targets(b).await?;
        }
        None => {
            let f = stdin();
            let b = BufReader::new(f);
            scrape_targets(b).await?;
        }
    }

    Ok(())
}

async fn scrape_targets<R: async_std::io::Read + Unpin>(
    reader: async_std::io::BufReader<R>,
) -> Result<(), Box<dyn Error>> {
    for line in reader.lines().next().await {
        let line = line?;
        log::debug!("scraping {:?}", line);
        let response = surf::get(line).recv_string().await?;
        log::debug!("response {:?}", response);
    }
    Ok(())
}
