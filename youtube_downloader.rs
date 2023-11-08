use std::env;
use youtube_dl::{Options, Video};

#[tokio::main]
async fn main() {
    // Get the YouTube video URL from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: youtube_downloader <video_url>");
        return;
    }

    let video_url = &args[1];

    // Create YouTubeDL options
    let options = Options {
        all: false,
        format: Some("bestvideo+bestaudio/best".to_string()),
        ..Default::default()
    };

    // Fetch video information
    let video_info_result = youtube_dl::get_info(video_url, options).await;

    match video_info_result {
        Ok(video_info) => {
            // Print video information
            print_video_info(&video_info);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}

fn print_video_info(video_info: &Video) {
    println!("Title: {}", video_info.title);
    println!("Author: {}", video_info.uploader);
    println!("Duration: {} seconds", video_info.duration);
    println!("URL: {}", video_info.url);
}
