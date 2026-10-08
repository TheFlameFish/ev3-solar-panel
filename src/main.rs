extern crate ev3dev_lang_rust;
extern crate image;
extern crate imageproc;
extern crate reqwest;

extern crate lazy_static;

use std::{thread, time};

use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::sound;
use ev3dev_lang_rust::{Button, Ev3Result};

const YAW_INCR: i32 = 15;
const PITCH_INCR: i32 = 5;

const YAW_MIN: i32 = -90;
const YAW_MAX: i32 = 90;

const PITCH_MIN: i32 = 0;
const PITCH_MAX: i32 = 30;

const TIME_BUFFER: time::Duration = time::Duration::from_millis(500);

fn main() -> Ev3Result<()> {
    let yaw_motor = LargeMotor::get(MotorPort::OutA)?;
    let pitch_motor = LargeMotor::get(MotorPort::OutB)?;
    let button = Button::new()?;

    sound::beep()?;
    sound::speak("Hello, I am Robot")?.wait()?;

    // Set the initial speed so that the motors will move
    yaw_motor.set_speed_sp(100)?;
    pitch_motor.set_speed_sp(100)?;
    yaw_motor.set_stop_action("hold")?;
    pitch_motor.set_stop_action("hold")?;
    yaw_motor.set_position(0)?;
    pitch_motor.set_position(0)?;

    let mut yaw = YAW_MIN;
    let mut pitch = PITCH_MIN;

    while yaw <= YAW_MAX {
        while pitch <= PITCH_MAX {   
            yaw_motor.run_to_abs_pos(Some(yaw))?;
            #[cfg(target_os = "linux")]
            yaw_motor.wait_until_not_moving(None);
            pitch_motor.run_to_abs_pos(Some(pitch))?;
            #[cfg(target_os = "linux")]
            pitch_motor.wait_until_not_moving(None);

            let speech = format!("Currently at angle: {} yaw, {} pitch.", yaw, pitch);

            thread::sleep(TIME_BUFFER);

            loop {
                button.process();
                if button.is_up() {
                    break;
                } else if button.is_down() {
                    sound::speak(&speech)?.wait()?;
                }
            }

            pitch += PITCH_INCR;
        }
        yaw += YAW_INCR;
        pitch = PITCH_MIN;
    }

    Ok(())
}
