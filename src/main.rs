use clap::Parser;
use directories::UserDirs;
use manic::{Downloader, Hash, ProgressStyle};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct AppArgs {
    /// url site to dowload
    #[arg(short, long)]
    url: String,
    /// total concurent spawned while downloading
    #[arg(short, long, default_value_t = 4)]
    worker: u8,
    /// hash value to verify checksum [OPTIONAL]
    #[arg(short, long)]
    checksum: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), manic::ManicError> {
    let args = AppArgs::parse();

    let mut client = Downloader::new(&args.url, args.worker).await?;

    if let Some(hash) = args.checksum {
        let hash_str = Hash::new_sha256(hash);
        client.verify(hash_str);
    }

    let user_dir = UserDirs::new().expect("Your rig should have Users Folder");
    let download_dir = user_dir
        .download_dir()
        .expect("Your rig should have Download Folder");
    let path_download = download_dir
        .to_str()
        .expect("should have convert to string");

    client.bar_style(ProgressStyle::default_bar());

    client.progress_bar().download_and_save(path_download).await?;

    Ok(())
}
