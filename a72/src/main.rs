use chrono::Local;

extern crate chan_signal;

use chan_signal::Signal;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
#[macro_use]
extern crate log;
extern crate env_logger;
use std::pin;
use std::thread;
use std::time::Duration;
fn main() {
    // let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    // dbg!(time);
    //
    // let signal = chan_signal::notify(&[Signal::INT, Signal::TERM, Signal::HUP, Signal::QUIT]);
    //
    // if let Some(signal) = signal.recv() {
    //     println!("received signal: {:?}", signal)
    // }
    //
    // println!("111");
    env_logger::init();

    info!("info level msg");
    warn!("warn level msg");
    error!("error level msg");
    let p = pin_slice();
    println!("{:?}", p.as_ptr());

    let mut vec = Vec::with_capacity(10);
    vec.extend([1, 2, 3].iter().cloned());
    assert_eq!(vec.capacity(), 10);
    vec.shrink_to_fit();
    println!("{:?}", vec.capacity());
}

fn pin_slice() -> std::pin::Pin<Box<[u8]>> {
    Box::pin([0; 1024])
}

fn postgress() {
    let m = MultiProgress::new();
    let sty = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .progress_chars("##-");

    let pb = m.add(ProgressBar::new(128));
    pb.set_style(sty.clone());
    let _ = thread::spawn(move || {
        for i in 0..128 {
            pb.set_message(&format!("item #{}", i + 1));
            pb.inc(1);
            thread::sleep(Duration::from_millis(15));
        }
        pb.finish_with_message("done");
    });

    let pb = m.add(ProgressBar::new(128));
    pb.set_style(sty.clone());
    let _ = thread::spawn(move || {
        for _ in 0..3 {
            pb.set_position(0);
            for i in 0..128 {
                pb.set_message(&format!("item #{}", i + 1));
                pb.inc(1);
                thread::sleep(Duration::from_millis(8));
            }
        }
        pb.finish_with_message("done");
    });

    let pb = m.add(ProgressBar::new(1024));
    pb.set_style(sty.clone());
    let _ = thread::spawn(move || {
        for i in 0..1024 {
            pb.set_message(&format!("item #{}", i + 1));
            pb.inc(1);
            thread::sleep(Duration::from_millis(2));
        }
        pb.finish_with_message("done");
    });

    m.join_and_clear().unwrap();
}
