use defmt::*;
use embassy_stm32::peripherals::TIM2;
use embassy_stm32::timer::simple_pwm::SimplePwmChannel;
use embassy_sync::channel::Receiver;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use crate::data_types::{ServoInput, ServoDir};

const SERVO_CENTER_DUTY: u16 = 1500;

#[embassy_executor::task]
pub async fn servo_controller_task(
    mut servo: SimplePwmChannel<'static, TIM2>,
    input_sub: Receiver<'static, NoopRawMutex, ServoInput, 1>) {
    loop {
        let input = input_sub.receive().await;
        info!("received input for servo");

        let duty: u16 = 50 * input.angle as u16 / 9;
        match input.dir {
            ServoDir::Left => {
                servo.set_duty_cycle(SERVO_CENTER_DUTY - duty);
            },
            ServoDir::Right => {
                servo.set_duty_cycle(SERVO_CENTER_DUTY + duty);
            },
            ServoDir::Middle => {
                servo.set_duty_cycle(SERVO_CENTER_DUTY);
            }
        }
    }
}

