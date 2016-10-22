extern crate hyper;
extern crate rss;

use rss::Channel;
use std::str::FromStr;

use broadcast::Broadcaster;

pub struct BassDrive {
    pub url: String
}

impl BassDrive {
}

impl Broadcaster for BassDrive {
    fn broadcast(&self) -> Channel {
        Channel::from_str("").unwrap()
    }
}
