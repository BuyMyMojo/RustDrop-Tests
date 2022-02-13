use std::{thread, time};
use clap::Parser;
use std::path::Path;
use pbr::ProgressBar;
use rand::{thread_rng, Rng};
use comfy_table::*;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to video or folder of images.
    #[clap(short, long)]
    video: String,

    /// Used to set png or tiff.
    /// tiff is faster.
    #[clap(short, long, default_value = "tiff")]
    format: String,

    /// Number of frames in "video".
    /// Used for fake data.
    #[clap(long, default_value_t = 18000)]
    frames: u64,
}

fn main() {
    let args = Args::parse();
    let mut rng = thread_rng();
    let file_path = Path::new(&args.video);

    println!("Generating {} tiff images from: {}", args.frames, file_path.display());

    // Fake image generation
    let mut image_progress = ProgressBar::new(args.frames);
    image_progress.format("╢▌▌░╟");
    for _ in 0..args.frames {
        thread::sleep(time::Duration::from_millis(69));
        image_progress.inc();
    }
    image_progress.finish_print("Finished generating images!");
    thread::sleep(time::Duration::from_secs(3));

    println!();
    println!("Determining FPS from frames");

    // Fake image counting
    let mut frame_count_progress = ProgressBar::new(args.frames);
    frame_count_progress.format("╢▌▌░╟");

    let mut new_frames = Vec::new();

    for frame in 0..args.frames {
        thread::sleep(time::Duration::from_millis(69));
        let is_new_frame = rng.gen_range(0..2);

        new_frames.push([frame, is_new_frame]);

        frame_count_progress.inc();
    }
    frame_count_progress.finish_print("Finished getting frame rate!");
    thread::sleep(time::Duration::from_secs(3));

    println!();
    println!("Generating table from data");

    // Table generation
    let mut table_gen_progress = ProgressBar::new(args.frames);
    table_gen_progress.format("╢▌▌░╟");

    let mut only_new_frames = Vec::new();
    let mut only_dupe_frames = Vec::new();

    for data in new_frames {
        thread::sleep(time::Duration::from_millis(69));
        match data[1] {
            1 => only_new_frames.push(true),
            0 => only_dupe_frames.push(false),
            _ => panic!("Not a bool!"),
        }
        table_gen_progress.inc();
    }
    table_gen_progress.finish_print("Finished generating table!");
    thread::sleep(time::Duration::from_secs(3));
    println!();
    let mut table = Table::new();
    table.load_preset(comfy_table::presets::UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_table_width(100)
        .set_header(
            vec![Cell::new("Dupe Frames"),
                 Cell::new("New Frames")
            ])
        .add_row(
            vec![
                Cell::new(only_dupe_frames.len()).fg(Color::Red),
                Cell::new(only_new_frames.len()).fg(Color::Green)
            ]);

    println!("{table}")

}
