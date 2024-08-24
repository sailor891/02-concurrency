use anyhow::Result;
use core::fmt;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

struct Msg {
    idx: usize,
    msg: u32,
}
fn main() -> Result<()> {
    let (tx, rx) = channel();

    for i in 0..5 {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);

    let handle = thread::spawn(move || consumer(rx));
    let c = handle
        .join()
        .map_err(|e| anyhow::anyhow!("thread error{:?}", e));
    println!("{:?}", c);

    Ok(())
}
fn producer(idx: usize, tx: Sender<Msg>) {
    loop {
        let msg = Msg::new(idx, rand::random::<u32>());
        tx.send(msg).unwrap();
        thread::sleep(Duration::from_millis(1000));
        if rand::random::<u8>() % 5 == 0 {
            break;
        }
    }
    println!("thread {} exit", idx);
}
fn consumer(rx: Receiver<Msg>) -> String {
    for msg in rx {
        println!("{:?}", msg);
    }
    "consumer exit".to_string()
}
impl Msg {
    fn new(idx: usize, msg: u32) -> Msg {
        Msg { idx, msg }
    }
}
impl fmt::Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.idx, self.msg)
    }
}
impl fmt::Debug for Msg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "msg:{}", self.msg)
    }
}
