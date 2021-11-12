# rcut

## Random audio sample generator

Generates `n` samples from directory `i` of `l` length from `t` format to `o` directory. Good for beatmaking and not much else. Requires `ffmpeg` and `ffprobe` on the `PATH`.

```
cargo run --release -- -i test -l 10 -n 3 -t mkv -o out
```
