# Node Lab

Node Lab is an experimental node-based raster image editor written in Rust.  The project is in its earliest stages and currently offers a proof‑of‑concept UI built with `eframe`/`egui`.

## Project Goals

The long term goal is to allow image manipulation through a graph of nodes.  Each node represents an operation (such as blending, filtering, or color adjustment) and outputs a raster image.  Users will be able to connect nodes together to compose complex effects.

At the moment the application only demonstrates basic node interaction:

- Two draggable boxes that represent nodes
- Connection points on each box
- The ability to draw a line between connection points

This groundwork will eventually evolve into a fully featured editor where nodes process images and connections describe the flow of pixel data.

## Building and Running

This repository is a standard Rust crate.  To build the example UI, use Cargo:

```bash
cargo run
```

The first build may take a while as dependencies are fetched.  After compiling, a window will open displaying two boxes that can be moved around and connected.

### Requirements

- Rust toolchain (edition 2021 or later) – <https://www.rust-lang.org/tools/install>

## License

This project is released under the terms of the GNU General Public License (see the `LICENSE` file for details).

## Contributing

Contributions are welcome!  Because the editor is still in its infancy, there is plenty of room for improvement and experimentation.  Feel free to file issues or open pull requests.

