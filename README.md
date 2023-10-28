# audiobook-split-ffmpeg-rs
Split audiobook by chapters with the help of ffmpeg (rust version)

I wanted to learn Rust, so I needed a toy project. This repo contains
a re-implementation of my audiobook splitting tool, which I've previously
implemented in both Python and Golang.


Similar projects:
- https://github.com/MawKKe/audiobook-split-ffmpeg (python)
- https://github.com/MawKKe/audiobook-split-ffmpeg-go (golang)

*NOTE*: I recommend using the Golang version, as it is the most featureful. I have currently no plans to refine this Rust version to feature-parity with the other projects.

# Usage

Clone the repo. Then build the application:

    $ cargo build

Now the tool is available in `target/debug/audiobook-split-ffmpeg-rs`

Run the help to see how to use the tool:

    $ cargo run -q -- -h

or

    $ ./target/debug/audiobook-split-ffmpeg-rs -h

# Testing

Internal (unit) tests:

    $ cargo test

(testing is not really exhaustive, TODO)

CLI usage test/demo:

    $ make demo


# Dependencies

The third party Rust dependencies are handled automatically by `cargo`.

The application expects `ffprobe` and `ffmpeg` to be installed on your system, and
to be available via your `$PATH`.

# Roadmap

- [x] Implement subprocess call (ffprobe)
- [x] Implement JSON -> Chapters parsing
- [x] Implement CLI argument handling
    - [x] Input file
    - [x] Output directory
    - [x] Customization options, see audiobook-ffmpeg-split-go
- [x] Implement Chapters -> WorkItem conversion
- [x] Implement WorkItem + Option -> ffmpeg argument list conversion
- [ ] Implement subprocess call parallelization

# License

Copyright 2022 Markus Holmström (MawKKe)

The works under this repository are licenced under Apache License 2.0.
See file `LICENSE` for more information.

# Contributing

This project is hosted at https://github.com/MawKKe/audiobook-split-ffmpeg-rs

You are welcome to leave bug reports, fixes and feature requests. Thanks!



