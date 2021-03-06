#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate rustls;

use rustls::internal::msgs::codec::Reader;
use rustls::internal::msgs::fragmenter;
use rustls::internal::msgs::message;
use std::collections::VecDeque;
use std::convert::TryFrom;

fuzz_target!(|data: &[u8]| {
    let mut rdr = Reader::init(data);
    let msg = match message::OpaqueMessage::read(&mut rdr) {
        Ok(msg) => msg,
        Err(_) => return,
    };

    let msg = match message::Message::try_from(msg) {
        Ok(msg) => msg,
        Err(_) => return,
    };

    let frg = fragmenter::MessageFragmenter::new(5);
    let mut out = VecDeque::new();
    frg.fragment(message::OpaqueMessage::from(msg), &mut out);

    for msg in out {
        message::Message::try_from(msg).ok();
    }
});
