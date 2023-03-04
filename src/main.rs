// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

mod helloworld {
    include!(concat!(env!("OUT_DIR"), "/helloworld.skel.rs"));
}

use std::{
    thread::{self, sleep_ms},
    time::Duration,
};

use anyhow::Result;

use helloworld::*;
use libbpf_rs::{get_print, set_print, PrintLevel};

fn print_to_stdout(level: PrintLevel, msg: String) {
    println!("{msg}")
}

fn main() -> Result<()> {
    println!("hello from rust uwu");
    let mut skel_builder = HelloworldSkelBuilder::default();

    let mut open_skel = skel_builder.open()?;

    let mut skel = open_skel.load()?;
    skel.attach()?;
    // let p = get_print().unwrap();
    set_print(Some((PrintLevel::Debug, print_to_stdout)));
    let p = get_print().unwrap();
    eprintln!("{:?}", p);
    thread::sleep(Duration::from_secs(20));
    Ok(())
}
