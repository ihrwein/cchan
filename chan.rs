#![feature(unsafe_destructor)]

use std::thread;
use std::thread::JoinGuard;
use std::sync::mpsc::{channel, Sender};

pub struct ChannelWrapper<'a> {
    pub tx: Sender<Option<i32>>,
    pub guard: JoinGuard<'a, ()>
}

impl<'a> ChannelWrapper<'a> {

    pub fn new(tx: Sender<Option<i32>>, guard: JoinGuard<'a, ()>) -> ChannelWrapper<'a> {
        ChannelWrapper{tx: tx, guard: guard}    
    }

    pub fn send(&self, value: i32) -> bool {
        let res = self.tx.send(Some(value));

        match res {
            Ok(_) => true,
            Err(_)  => false,
        }
    }
}

#[unsafe_destructor]
impl <'a> Drop for ChannelWrapper<'a> {
    
    fn drop(&mut self) {
        self.tx.send(Option::None).unwrap();
    }
}

#[no_mangle]
pub extern fn channel_wrapper_create<'a>() -> Box<ChannelWrapper<'a>>
{
    println!("channel_wrapper_create()");

    let (tx, rx) = channel();
    let guard: JoinGuard<()> = thread::scoped(move|| {
        println!("worker thread: thread started");

        for i in rx.iter() {
            match i {
                Some(v) => println!("worker thread: received value: {:?}", v),
                None => {
                    println!("worker thread: received last value");
                    break;
                }
            };
        }

        println!("worker thread: thread finished");
    });

    Box::new(ChannelWrapper::new(tx, guard))
}

#[no_mangle]
pub extern fn channel_wrapper_send<'a>(this: &ChannelWrapper<'a>, value: i32) -> bool
{
    println!("channel_wrapper_send({:?})", value);
    this.send(value)
}

#[no_mangle]
pub extern fn channel_wrapper_free<'a>(_: Box<ChannelWrapper<'a>>)
{
    println!("channel_wrapper_free()");
}

#[allow(dead_code)]
fn main()
{
    let cw = channel_wrapper_create();

    for i in 0..100 {
        channel_wrapper_send(&*cw, i);
    }

    channel_wrapper_free(cw);
}
