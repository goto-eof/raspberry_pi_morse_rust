mod morse_service;
use std::thread;
use std::time::Duration;

use crate::morse_service::LONG;
use crate::morse_service::PAUSE;
use crate::morse_service::PAUSE_BETWEEN_MORSE_SIGNALS;
use crate::morse_service::SHORT;
use morse_service::translate;
use rust_gpiozero::*;

const MESSAGE: &str = "Hello World!";
fn main() {
    let led = LED::new(17);

    let result = translate(MESSAGE);
    if result.is_err() {
        println!("{}", result.err().unwrap());
        return;
    }
    let morse: Vec<u32> = result.unwrap();
    print_message(&morse);
    loop {
        for morse_value in morse.clone() {
            led.on();
            println!("ON");
            thread::sleep(Duration::from_millis(morse_value as u64));
            led.off();
            println!("OFF");
            thread::sleep(Duration::from_millis(PAUSE_BETWEEN_MORSE_SIGNALS as u64));
        }
    }
}

fn print_message(morse: &[u32]) {
    println!("Sending: {}", MESSAGE);
    print!("Translated: ");
    for morse_value in morse.iter() {
        if *morse_value == SHORT {
            println!(".")
        } else if *morse_value == LONG {
            println!("_")
        } else if *morse_value == PAUSE {
            println!(" ")
        }
    }
    println!("Blinking...");
}
