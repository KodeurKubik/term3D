# Term3D

A Vertex + Edge 3D renderer in the terminal, written in rust 👍

Well... Yeah :) Basically put 3D vertices in `data.rs` and export them at the bottom of the function and run it :D

## Demo

https://github.com/user-attachments/assets/f2a90afa-381c-40cc-84c5-c81444dc4478

## Running

its rust so pretty easy

```sh
# debug mode
cargo run

# compile it
cargo build --release
./target/release/term3d
```

## By the way

You have to change the config of your terminal to display squares for every character, I did so on Ghostty (in the config):
```yaml
# For PERFECT SQUARE characters
font-family = "JetBrainsMono NFM"
font-style = "Normal"
adjust-cell-width = 0%
adjust-cell-height = -55%
```

Thank you Mattbatwings for the inspiration: https://www.youtube.com/watch?v=hFRlnNci3Rs (go check him out)

that's it have fun like I did when coding this

**MIT Licensed - © Kodeur_Kubik**
