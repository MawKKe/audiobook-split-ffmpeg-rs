# audiobook-split-ffmpeg-rs
Split audiobook by chapters with the help of ffmpeg (rust version)

Similar projects:
- https://github.com/MawKKe/audiobook-split-ffmpeg (python)
- https://github.com/MawKKe/audiobook-split-ffmpeg-go (golang)


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
