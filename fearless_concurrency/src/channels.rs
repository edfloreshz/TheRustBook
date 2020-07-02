//  Created by Eduardo Flores on 5/18/20.
//  edfloreshz@gmail.com

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

pub fn create_channels() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
       let values = vec![
           String::from("Hi"),
           String::from("there"),
           String::from("from"),
           String::from("spawned"),
           String::from("thread"),
       ];
        for item in values {
            tx.send(item).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });

    for item in rx {
        println!("Got: {} ", item)
    }
}