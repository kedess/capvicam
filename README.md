## capvicam

Utility for capturing video from a USB camera via the video for linux system (v4l2)


### Usage example:
Run:
```bash
RUST_LOG=info ./target/release/capvicam --path=/dev/video0 --width=1920 --height=1080 --mjpeg=enable --port=8008
```
Watching videos in the browser:
```bash
http://localhost:8008
```
Watching videos via ffplay:
```bash
ffplay http://localhost:8008
```

Getting help:
```bash
./target/release/capvicam -h
```