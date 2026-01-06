use baad_dm::{Download, Downloader, DownloaderConfig, StyleOptions};
use std::path::PathBuf;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let url = "http://speed.transip.nl/random-100mb.bin";
    let file_path = PathBuf::from("test_downloads/random-100mb.bin");

    let _ = std::fs::remove_file(&file_path);
    let _ = std::fs::create_dir_all("test_downloads");

    println!("Test 1: Single stream download...");
    let _ = std::fs::remove_file(&file_path);
    
    let config = DownloaderConfig::builder()
        .directory(PathBuf::from("test_downloads"))
        .concurrent_downloads(1)
        .chunk_threshold(1024 * 1024 * 1024) // 1GB threshold = no chunking
        .style_options(StyleOptions::hidden())
        .overwrite(true)
        .build();

    let downloader = Downloader::new(config);
    let downloads = vec![Download::try_from(url).unwrap()];

    let start = Instant::now();
    let results = downloader.download(&downloads).await;
    let elapsed = start.elapsed();

    let size_mb = results[0].size as f64 / 1024.0 / 1024.0;
    println!("Status: {:?}", results[0].status);
    println!("Single stream: {:.2} MB in {:.2}s = {:.2} MB/s\n", size_mb, elapsed.as_secs_f64(), size_mb / elapsed.as_secs_f64());

    println!("Test 2: Auto chunking (8MB chunks, max 8 concurrent)...");
    let _ = std::fs::remove_file(&file_path);

    let config = DownloaderConfig::builder()
        .directory(PathBuf::from("test_downloads"))
        .concurrent_downloads(1)
        .max_chunks_per_file(16)
        .max_concurrent_chunks(8)
        .chunk_threshold(1024 * 1024) // 1MB threshold
        .style_options(StyleOptions::hidden())
        .overwrite(true)
        .build();

    let downloader = Downloader::new(config);
    let downloads = vec![Download::try_from(url).unwrap()];

    let start = Instant::now();
    let results = downloader.download(&downloads).await;
    let elapsed = start.elapsed();

    let size_mb = results[0].size as f64 / 1024.0 / 1024.0;
    println!("Status: {:?}", results[0].status);
    println!("Auto chunks (max 8 concurrent): {:.2} MB in {:.2}s = {:.2} MB/s\n", size_mb, elapsed.as_secs_f64(), size_mb / elapsed.as_secs_f64());

    println!("Test 3: Auto chunking (8MB chunks, max 16 concurrent)...");
    let _ = std::fs::remove_file(&file_path);

    let config = DownloaderConfig::builder()
        .directory(PathBuf::from("test_downloads"))
        .concurrent_downloads(1)
        .max_chunks_per_file(32)
        .max_concurrent_chunks(16)
        .chunk_threshold(1024 * 1024)
        .style_options(StyleOptions::hidden())
        .overwrite(true)
        .build();

    let downloader = Downloader::new(config);
    let downloads = vec![Download::try_from(url).unwrap()];

    let start = Instant::now();
    let results = downloader.download(&downloads).await;
    let elapsed = start.elapsed();

    let size_mb = results[0].size as f64 / 1024.0 / 1024.0;
    println!("Status: {:?}", results[0].status);
    println!("Auto chunks (max 16 concurrent): {:.2} MB in {:.2}s = {:.2} MB/s\n", size_mb, elapsed.as_secs_f64(), size_mb / elapsed.as_secs_f64());

    println!("Test 4: Default config...");
    let _ = std::fs::remove_file(&file_path);

    let config = DownloaderConfig::builder()
        .directory(PathBuf::from("test_downloads"))
        .style_options(StyleOptions::hidden())
        .overwrite(true)
        .build();

    let downloader = Downloader::new(config);
    let downloads = vec![Download::try_from(url).unwrap()];

    let start = Instant::now();
    let results = downloader.download(&downloads).await;
    let elapsed = start.elapsed();

    let size_mb = results[0].size as f64 / 1024.0 / 1024.0;
    println!("Status: {:?}", results[0].status);
    println!("Default config: {:.2} MB in {:.2}s = {:.2} MB/s", size_mb, elapsed.as_secs_f64(), size_mb / elapsed.as_secs_f64());

    let _ = std::fs::remove_file(&file_path);
}
