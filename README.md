# audiobook-split-ffmpeg-rs
Split audiobook by chapters with the help of ffmpeg (rust version)

Similar projects:
- https://github.com/MawKKe/audiobook-split-ffmpeg (python)
- https://github.com/MawKKe/audiobook-split-ffmpeg-go (golang)


# Roadmap

- [x] Implement subprocess call (ffprobe)
- [x] Implement JSON -> Chapters parsing
- [ ] Implement CLI argument handling
    - [x] Input file
    - [ ] Output directory
    - [ ] Customization options, see audiobook-ffmpeg-split-go
- [ ] Implement Chapters -> WorkItem conversion
- [ ] Implement WorkItem + Option -> ffmpeg argument list conversion
- [ ] Implement subprocess call parallelization
