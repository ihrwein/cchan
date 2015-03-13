#![feature(unsafe_destructor)]

use std::thread;
use std::thread::{Thread, JoinGuard};
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, SendError, Receiver};

pub struct ChannelWrapper<'a, T: Send + 'a> {
    pub tx: Sender<Option<T>>,
    pub handle: JoinGuard<'a, T>
}

impl<'a, T: Send> ChannelWrapper<'a, T> {

    pub fn send(&self, value: T) -> bool {
        let res = self.tx.send(Some(value));

        match res {
            Ok(v) => true,
            Err(e)  => false,
        }
    }
}

#[unsafe_destructor]
impl <'a, T: Send> Drop for ChannelWrapper<'a, T> {
    
    fn drop(&mut self) {
        self.tx.send(Option::None).unwrap();
    }
}

    
fn main()
{
    let (tx, rx) = channel();
    let consumer = thread::spawn(move|| {
        println!("thread started");
        for i in rx.iter() {
            match i {
                Some(v) => println!("{:?}", v),
                None => {
                    println!("received no more");
                    break;
                }
            };
        }
        println!("thread finished");
    });

    for i  in 0..10{
        println!("sending: {:?}", i);
        tx.send(Option::Some(i)).unwrap();
    }

    tx.send(Option::None).unwrap();

    let _ = consumer.join();
}
