# Ninve

Ninve (Ninve Is Not a Video Editor) is a simple (MPV wrapper) command-line tool written in Rust that allows you to quickly trim videos in a lossless manner using a text-based user interface (TUI). It uses `mpv` and `ffmpeg` to provide an efficient and user-friendly video trimming experience.

## Features
- Lossless video trimming
- Text-based user interface (TUI) for easy navigation
- Simple command-line usage
- Lightweight and fast, built with Rust

## Prerequisites
To use Ninve, you need the following dependencies installed on your system:
- `mpv`: A free, open-source media player
- `ffmpeg`: A powerful multimedia framework for handling video and audio

Ensure these binaries are available in your system's PATH.

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/ninve.git
   ```
2. Navigate to the project directory:
   ```bash
   cd ninve
   ```
3. Build the project using Cargo:
   ```bash
   cargo build --release
   ```
4. The binary will be available in `target/release/ninve`.

## Usage
Run Ninve by providing the path to the input video file:

```bash
ninve ./path-to-my-file.mp4
```

Optionally, you can specify an output path for the trimmed video:

```bash
ninve ./path-to-my-file.mp4 ./output-file.mp4
```

The TUI will guide you through selecting the start and end points for trimming the video.

## How it works


https://github.com/user-attachments/assets/6658d4de-6f88-4d55-9c1b-3ca528ab4e4b



## Contributing
Contributions are welcome! Please feel free to submit issues or pull requests to the repository.

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.
