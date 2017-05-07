#[macro_use] extern crate clap;
#[macro_use] extern crate error_chain;

mod app;
mod args;
mod error;

quick_main!(app::start);
