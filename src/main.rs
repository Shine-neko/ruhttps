#![allow(unused_variables, unused_must_use)]
#[macro_use]
extern crate sozu_lib as sozu;
#[macro_use]
extern crate sozu_command_lib as sozu_command;
extern crate time;

use sozu_command::channel::Channel;
use sozu_command::logging::{Logger, LoggerBackend};
use sozu_command::proxy;
use sozu_command::proxy::LoadBalancingParams;
use std::env;
use std::io::stdout;
use std::thread;

fn main() {
    Logger::init(
        "EXAMPLE".to_string(),
        "debug",
        LoggerBackend::Stdout(stdout()),
        None,
    );

    let config = proxy::HttpsListener {
        front: "127.0.0.1:8443".parse().expect("could not parse address"),
        cipher_list: String::from(
            "ECDHE-ECDSA-CHACHA20-POLY1305:\
    ECDHE-RSA-CHACHA20-POLY1305:ECDHE-ECDSA-AES128-GCM-SHA256:\
    ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:\
    ECDHE-RSA-AES256-GCM-SHA384:DHE-RSA-AES128-GCM-SHA256:\
    DHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-AES128-SHA256:\
    ECDHE-RSA-AES128-SHA256:ECDHE-ECDSA-AES128-SHA:\
    ECDHE-RSA-AES256-SHA384:ECDHE-RSA-AES128-SHA:\
    ECDHE-ECDSA-AES256-SHA384:ECDHE-ECDSA-AES256-SHA:\
    ECDHE-RSA-AES256-SHA:DHE-RSA-AES128-SHA256:DHE-RSA-AES128-SHA:\
    DHE-RSA-AES256-SHA256:DHE-RSA-AES256-SHA:ECDHE-ECDSA-DES-CBC3-SHA:\
    ECDHE-RSA-DES-CBC3-SHA:EDH-RSA-DES-CBC3-SHA:AES128-GCM-SHA256:\
    AES256-GCM-SHA384:AES128-SHA256:AES256-SHA256:AES128-SHA:\
    AES256-SHA:DES-CBC3-SHA:!DSS",
        ),

        ..Default::default()
    };

    let (mut command, channel) = Channel::generate(1000, 10000).expect("should create a channel");
    let jg = thread::spawn(move || {
        let max_buffers = 500;
        let buffer_size = 16384;
        sozu::https_rustls::configuration::start(config, channel, max_buffers, buffer_size);
    });

    let _ = jg.join();
}
