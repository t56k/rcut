# rcut

## Random audio track sample (RATS) generator

Generates `s` samples from directory `i` of `l` length from `t` format to `o` directory. Good for beatmaking and not much else. Requires `ffmpeg` and `ffprobe` on the `PATH`.

```
cargo run --release -- -i input_dir -l 0.9 -t mkv -s 5 -o output_dir
```

### Todo

- Make necessary dirs on run
