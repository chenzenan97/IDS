mod lib;

use lib::{Rectangle, iou};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();



    let r1 = Rectangle {
        x1: args[1].parse().unwrap(),
        y1: args[2].parse().unwrap(),
        x2: args[3].parse().unwrap(),
        y2: args[4].parse().unwrap(),
    };
    let r2 = Rectangle {
        x1: args[5].parse().unwrap(),
        y1: args[6].parse().unwrap(),
        x2: args[7].parse().unwrap(),
        y2: args[8].parse().unwrap(),
    };

    println!("IoU value: {}", iou(&r1, &r2));
}