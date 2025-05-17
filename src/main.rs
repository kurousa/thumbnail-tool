use clap::Parser;
use std::{
    fs::{create_dir_all, read_dir, remove_dir_all},
    path::PathBuf,
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
    if let Err(e) = create_dir_all(&args.output) {
        eprintln!(
            "Error: Failed to create output directory {:?}: {}",
            args.output, e
        );
        std::process::exit(1);
    }

    // 1． スレッド処理を分割して、各スレッドが独立して処理を行う実装
    // Arc, Mutexを使って、スレッド間で共有するデータを管理する
    // use std::sync::{Arc, Mutex};
    // use std::thread;
    // let mut handles = vec![];
    // let mut all_paths = vec![];
    // for entry in read_dir(&args.input).unwrap() {
    //     let entry = entry.unwrap();
    //     let path = entry.path();
    //     if path.is_dir() {
    //         continue;
    //     }
    //     all_paths.push(path);
    // }

    // let processed_count = Arc::new(Mutex::new(0));
    // let chunk_size = (all_paths.len() + 3) / 4;

    // for chunk in all_paths.chunks(chunk_size) {
    //     let chunk = chunk.to_vec();
    //     let processed_count = processed_count.clone();
    //     let output = args.output.clone();
    //     handles.push(thread::spawn(move || {
    //         for path in chunk {
    //             let output_path = output.join(path.file_name().unwrap());
    //             let img = image::open(&path);
    //             if let Ok(img) = img {
    //                 let thumbnail = img.thumbnail(64, 64);
    //                 thumbnail.save(output_path).unwrap();
    //                 let mut writer = processed_count.lock().unwrap();
    //                 *writer += 1;
    //             }
    //         }
    //     }))
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!(
    //     "Processed {} images",
    //     processed_count.as_ref().lock().unwrap()
    // );

    // 2. チャンネルを使って、スレッド間でメッセージを送受信する実装
    // Producer-Consumerパターンを使用して、スレッド間でメッセージを送受信する
    //
    // use std::sync::mpsc::channel;
    // let mut handles = vec![];
    // let mut channels = vec![];
    // let (counter_tx, counter_rx) = channel::<usize>();

    // // recv側(サムネイル生成処理側)の立ち上げ
    // for _ in 0..4 {
    //     let (tx, rx) = channel::<PathBuf>();
    //     channels.push(tx);
    //     let counter_tx = counter_tx.clone();
    //     let output = args.output.clone();
    //     handles.push(thread::spawn(move || {
    //         while let Ok(path) = rx.recv() {
    //             let output_path = output.join(path.file_name().unwrap());
    //             let img = image::open(&path);
    //             if let Ok(img) = img {
    //                 let thumbnail = img.thumbnail(64, 64);
    //                 thumbnail.save(output_path).unwrap();
    //                 counter_tx.send(1).unwrap();
    //             }
    //         }
    //     }))
    // }

    // // send側(ファイル読み込み処理側)の立ち上げ
    // // 処理対象ファイルパスの送信
    // for (index, entry) in read_dir(&args.input).unwrap().enumerate() {
    //     let entry = entry.unwrap();
    //     let path = entry.path();
    //     if path.is_dir() {
    //         continue;
    //     }
    //     channels[index % channels.len()].send(path).unwrap();
    // }
    // // すべてのファイルを送信した後、各スレッドに終了を通知
    // for channel in channels {
    //     drop(channel);
    // }
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // let processed_count = counter_rx.iter().count();
    // println!("Processed {} images", processed_count);

    // 3. rayonを使って、並列処理を行う実装
    use rayon::prelude::*;
    let items: Vec<_> = read_dir(&args.input).unwrap().collect();
    let result = items.into_par_iter().map(|item| {
        let item = item.unwrap();
        let path = item.path();
        let output_path = args.output.join(path.file_name().unwrap());
        let img = image::open(&path);
        if let Ok(img) = img {
            let thumbnail = img.thumbnail(64, 64);
            thumbnail.save(&output_path).unwrap();
            1
        } else {
            0
        }
    });
    println!("Processed {} images", result.sum::<u32>());
}
