//! PIR motion detector
//!
//! Assumes the Passive Infrared (PIR) motion sensor is connected to GPIO4.
//! 

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    // declare pir sensor as input
    let mut pir = PinDriver::input(peripherals.pins.gpio4)?;

    // Ultrasonic sensor: HC-SR04
    // PIN_TRIG - output - gpio1 - pulse to start measurement
    let mut pin_trig = PinDriver::output(peripherals.pins.gpio1)?;
    // PIN_ECHO - input -  gpio2 - Measure the high pulse length to get the distance
    let mut pin_echo = PinDriver::input(peripherals.pins.gpio2)?;

    // loop {
    //   // // Start a new measurement:
    //   pin_trig.set_low()?;
    //   FreeRtos::delay_ms(2000);
    //   // digitalWrite(PIN_TRIG, HIGH);
    //   pin_trig.set_high()?;
    //   // delayMicroseconds(10);
    //   FreeRtos::delay_ms(10000);
    //   // digitalWrite(PIN_TRIG, LOW);
    //   pin_trig.set_low()?;

    //   // // Read the result:
    //   // int duration = pulseIn(PIN_ECHO, HIGH);
    //   // read HIGH pulse on PIN_ECHO, return length of pulse (us)

    //   // Serial.print("Distance in CM: ");
    //   // Serial.println(duration / 58);
    //   // Serial.print("Distance in inches: ");
    //   // Serial.println(duration / 148);
    // }
    

    println!("Ankylosaurid");
    loop {
        match pir.is_high() {
          true => {
            println!("Motion detected!!");
            FreeRtos::delay_ms(5000);
          },
          pir.is_low() => {
            println!("loowwoo")  
          }
          
        }
        FreeRtos::delay_ms(1000);
    }
}
