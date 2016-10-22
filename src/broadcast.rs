extern crate rss;
extern crate hyper;

use hyper::server::{Request, Response};
use rss::Channel;

pub trait Broadcaster {
    fn broadcast(&self) -> Channel;
}

#[allow(unused)]
pub fn build_broadcast<T: Broadcaster>(b: T) -> Box<Fn(Request, Response)> {
    let channel = b.broadcast();

    Box::new(move |_: Request, res: Response| {
        res.send(channel.to_string().as_bytes());
    })
}
