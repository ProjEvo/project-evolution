To run this project, first `cd` to the folder you want to download the project in. 
Make sure you have `git`, `cargo`, and `rustc` installed, and that the following command returns at least `1.65.0`:

```bash
rustc --version
```

After you've done that, clone the repo with:
```bash
git clone https://github.com/ProjEvo/project-evolution.git
```

cd into it:
```bash
cd project-evolution
```

To compile and run the project, run:
```
cargo run --release
```

> Note: `--release` is enabled significant optimizations that made it possible to run at 10x speed on my laptop. Without these optimizations, your mileage may vary.
