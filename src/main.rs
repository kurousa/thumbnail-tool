use clap::Parser;
use std::{
    fs::{create_dir_all, read_dir, remove_dir_all},
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};

#[derive(Parser)]
struct Args {
    input: PathBuf,
    output: PathBuf,
}

fn main() {
    let args = Args::parse();

    if !args.input.exists() {
        eprintln!("Input directory does not exist");
        return;
    }
    println!("Processing images from {:?}", args.input);

    if args.output.exists() {
        println!("Output directory already exists. Deleting...");
        if let Err(e) = remove_dir_all(&args.output) {
            eprintln!("Failed to delete output directory: {}", e);
            std::process::exit(1);
        }
    }
    create_dir_all(&args.output).unwrap();

    let mut all_paths = vec![];
    for entry in read_dir(&args.input).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        all_paths.push(path);
    }

    let processed_count = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let chunk_size = (all_paths.len() + 3) / 4;

    for chunk in all_paths.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let processed_count = processed_count.clone();
        let output = args.output.clone();
        handles.push(thread::spawn(move || {
            let mut local_count = 0;
            for path in chunk {
                let output_path = output.join(path.file_name().unwrap());
                let img = image::open(&path);
                if let Ok(img) = img {
                    let thumbnail = img.thumbnail(64, 64);
                    thumbnail.save(output_path).unwrap();
                    local_count += 1;
                }
            }
            let mut writer = processed_count.lock().unwrap();
            *writer += local_count;
        }))
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "Processed {} images",
        processed_count.as_ref().lock().unwrap()
    );
}
