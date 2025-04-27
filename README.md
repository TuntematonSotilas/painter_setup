# painter_setup

Painter Setup : An application to calculate photo ratios for a painter canvas

## Docs

Leptos doc : https://book.leptos.dev

Thaw UI doc : https://thawui.vercel.app

## Setup

* Install Perl : 
Windows : https://strawberryperl.com - 
Linux : `sudo dnf install perl`

* Install Rust : https://www.rust-lang.org/tools/install
* Install Trunk : `cargo install trunk`
* Install wasm target : `rustup target add wasm32-unknown-unknown`
* Install Cargo Leptos : `cargo install --locked cargo-leptos`

## Run 
    cargo leptos watch

## Lint
    cargo clippy

## Docker

* Build : docker build . -t painter_setup
* Run : docker run -p 8080:80 painter_setup
* Test : http://localhost:8080