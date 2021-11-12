# rcut

## Random audio sample generator

Generates `n` samples from a directory `s` of `l` length from `t` format. Good for beatmaking and not much else. Requires `ffmpeg` and `ffprobe` on the `PATH`.

```
cargo run --release -- -s test -l 10 -n 3 -t mkv
```
