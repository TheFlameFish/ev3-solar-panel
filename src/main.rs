extern crate ev3dev_lang_rust;
extern crate image;
extern crate imageproc;
extern crate reqwest;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::Ev3Result;

const YAW_INCR: i32 = 15;
const PITCH_INCR: i32 = 5;

const YAW_MIN: i32 = -90;
const YAW_MAX: i32 = 90;

const PITCH_MIN: i32 = 0;
const PITCH_MAX: i32 = 30;

fn main() -> Ev3Result<()> {
    let yaw_motor = LargeMotor::get(MotorPort::OutA)?;
    let pitch_motor = LargeMotor::get(MotorPort::OutB)?;

    // Set the initial speed so that the motors will move
    yaw_motor.set_speed_sp(100)?;
    pitch_motor.set_speed_sp(100)?;
    yaw_motor.set_stop_action("hold");
    pitch_motor.set_stop_action("hold");

    let yaw = 0;
    let pitch = 0;
    yaw_motor.run_to_rel_pos(Some(YAW_MIN))?;
    #[cfg(target_os = "linux")]
    yaw_motor.wait_until_not_moving(None);
    pitch_motor.run_to_rel_pos(Some(PITCH_MIN))?;
    #[cfg(target_os = "linux")]
    pitch_motor.wait_until_not_moving(None);
    loop {

    }

    Ok(())
}
