## capvicam

Утилита захвата видео с usb камеры через систему video for linux (v4l2)


### Пример использования:
Запуск:
```bash
RUST_LOG=info ./target/release/capvicam --path=/dev/video0 --width=1920 --height=1080 --mjpeg=enable --port=8008
```
Просмотр видео в браузере:
```bash
http://localhost:8008
```
Просмотр видео через ffplay:
```bash
ffplay http://localhost:8008
```

Получение справки по параметрам запуска:
```bash
./target/release/capvicam -h
```