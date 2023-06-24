use std::time::Duration;

use rppal::gpio::Gpio;

fn main() {
    let pin1 = Gpio::new()
        .expect("To construct GPIO")
        .get(23)
        .expect("To get pin");
    let pin2 = Gpio::new()
        .expect("To construct GPIO")
        .get(24)
        .expect("To get pin");
    loop {
        match pin1.read() {
            rppal::gpio::Level::Low => (),
            rppal::gpio::Level::High => set_brighness(1.0),
        }
        match pin2.read() {
            rppal::gpio::Level::Low => (),
            rppal::gpio::Level::High => set_brighness(0.0),
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn set_brighness(val: f32) {
    std::env::set_var("DISPLAY", ":0");
    let res = std::process::Command::new("xrandr")
        .args(["--output","HDMI-1"])
        .args(["--brightness",&format!("{:.2}", val)])
        .output();
    println!("{:?}", res);
}
