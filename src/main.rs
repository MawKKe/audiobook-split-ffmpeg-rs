use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author,version,about,long_about=None)]
struct Args {
    #[arg(short, long)]
    input_file: PathBuf,
}

mod ffmpeg_split;

fn main() {
    let args = Args::parse();

    let abspath = std::fs::canonicalize(args.input_file).unwrap();

    let output = match ffmpeg_split::ffprobe::read_chapters(&abspath) {
        Ok(v) => v,
        Err(e) => panic!("Could not parse JSON: {}", e),
    };

    if output.chapters.len() < 1 {
        println!("Input file contains no chapter metadata. Cannot continue.");
    } else {
        for chap in output.chapters {
            println!("{} - {}", chap.id, chap.tags["title"]);
        }
    }
}
