# Rust_moc_gui

## Goal
The goal of this project is to create a GUI in Rust using [egui]("https://crates.io/crates/egui") (more precisely [eframe]("https://crates.io/crates/eframe")) to make different operations on MOCs easier. This project also enables a user to create MOCs and view information about the MOC on a user-friendly UI.

## Features
- [X] The user can launch an operation on a MOC.
- - [X] All 1 moc operations are implemented.
- [X] The user can launch an operation between 2 MOCs.
- - [X] Sfold and Tfold operations are now implemented.
- - [X] All 2 moc operations are implemented.
- [X] The user can import space, time and spacetime MOCs.
- - [X] import can be a fits/json/ASCII file.
- [X] The user can choose the type of export.
- - [X] export can be a fits/json/ASCII file.
- [X] The user can generate MOCs.
- [X] Info about the MOC can now be displayed via the list.
- [X] MOC's mollweide projection is shown in info (SMOC only).

## Running

### Running locally
Make sure you are using the latest version of stable rust by running `rustup update`.
`cargo run --release`

On Linux you need to first run:
`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:
`dnf install clang clang-devel clang-tools-extra speech-dispatcher-devel libxkbcommon-devel pkg-config openssl-devel libxcb-devel fontconfig-devel`

### Running Web Locally

We use [Trunk](https://trunkrs.dev/) to build for web target.
1. Install Trunk with `cargo install --locked trunk`.
2. Run `trunk serve` to build and serve on `http://127.0.0.1:8080`. Trunk will rebuild automatically if you edit the project.
3. Open `http://127.0.0.1:8080/index.html#dev` in a browser. See the warning below.
