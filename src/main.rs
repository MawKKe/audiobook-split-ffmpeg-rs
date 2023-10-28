use clap::Parser;
use std::os::unix::prelude::OsStrExt;
use std::path::PathBuf;
use std::thread::available_parallelism;

use path_clean::PathClean;

#[derive(Parser, Debug)]
#[command(author,version,about,long_about=None)]
struct Args {
    #[arg(short, long, help = "Path to your audiobook file")]
    input_file: PathBuf,

    #[arg(short, long, help = "Path to directory where split chapters are written")]
    output_dir: PathBuf,

    #[arg(
        short,
        long,
        help = "Maximum number of parallel ffmpeg jobs.\nTo use all CPUs, pass value 0 or leave the option unspecified"
    )]
    jobs: Option<u32>,

    #[arg(long, help = "List chapters in input file, then exit")]
    only_show_chapters: bool,

    #[arg(long, help = "List ffmpeg commands for extracting chapters, but don't execute them")]
    only_show_commands: bool,

    #[arg(long, help = "Don't use chapter titles in output filenames")]
    no_use_title_as_name: bool,

    #[arg(
        long,
        required = false,
        help = "Use this filename extension for chapters, instead of the one used by input file"
    )]
    use_output_extension: Option<String>,

    #[arg(long)]
    debug: bool,
}

impl Args {
    #[allow(dead_code)]
    fn max_jobs(&self) -> u32 {
        let max_cpus = available_parallelism().unwrap().get() as u32;
        let n = self.jobs.unwrap_or(max_cpus);
        if n == 0 || n > max_cpus {
            return max_cpus;
        }
        return n;
    }
}

mod ffmpeg_split;

fn main() {
    let args = Args::parse();

    let infile_abs = args.input_file.clean();

    let meta = match ffmpeg_split::ffprobe::read_chapters(&infile_abs, args.debug) {
        Ok(v) => v,
        Err(e) => panic!("Could not parse JSON: {}", e),
    };

    if meta.chapters.len() < 1 {
        println!("Input file contains no chapter metadata. Cannot continue.");
        std::process::exit(-1);
    }

    let mut opts = ffmpeg_split::ffprobe::Opts::new(meta.chapters.iter().max_by_key(|e| e.id).unwrap().id as usize);

    opts.use_title_as_name(!args.no_use_title_as_name);

    if args.only_show_chapters {
        // compute chaptern number width

        for chap in meta.chapters {
            println!(
                "{:0width$} - \"{}\"",
                chap.id,
                chap.tags.get("title").unwrap_or(&"<unknown chapter title>".to_string()),
                width = opts.chapter_id_formatted_width(),
            );
        }
        std::process::exit(0);
    }

    if args.only_show_commands {
        println!("Early exit after commands constructed");
        std::process::exit(0);
    }

    let out = &args.output_dir.clean();

    match std::fs::create_dir_all(out) {
        Ok(_) => (),
        Err(e) => panic!("Failed to create directory {:?}: {}", out, e),
    };

    println!("Created directory: {:?}", &out);

    for ch in meta.chapters {
        let wi = ffmpeg_split::ffprobe::make_workitem(&infile_abs, &out, &opts, &ch);

        let mut cmd = wi.to_cmd(&opts);

        let output = match cmd.output() {
            Ok(v) => v,
            Err(e) => panic!("Calling ffmpeg failed: {}", e),
        };

        if !output.status.success() {
            println!("ERROR: {:?}", std::ffi::OsStr::from_bytes(&output.stderr));
            std::process::exit(-1);
        }

        println!("processed: {:?}", wi.output_file);
    }

    std::process::exit(-1);
}
