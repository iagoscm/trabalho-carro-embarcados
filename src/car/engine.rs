use crate::common::{Direction, Car};
use rppal::gpio::{Gpio, OutputPin};

pub struct EngineControl {
    motor_pot_pin: OutputPin,
    brake_int_pin: OutputPin,
}

impl EngineControl {
    pub fn new(car: Car) -> Self {
        let gpio = Gpio::new().unwrap();

        EngineControl {
            motor_pot_pin: gpio.get(23).unwrap().into_output_low(),
            brake_int_pin: gpio.get(24).unwrap().into_output_low(),
        }
    }

    pub fn set_acceleration(&mut self, duty_cycle: f64) {
        self.motor_pot_pin.set_pwm_frequency(1000.0, duty_cycle).unwrap();
    }

    pub fn set_braking(&mut self, duty_cycle: f64) {
        self.brake_int_pin.set_pwm_frequency(1000.0, duty_cycle).unwrap();
    }

    pub fn set_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Idle => {
                self.set_acceleration(0.0);
                self.set_braking(0.0);
            }
            Direction::Accelerate => {
                self.set_acceleration(1.0);
                self.set_braking(0.0);
            }
            Direction::Reverse => {
                self.set_acceleration(0.0);
                self.set_braking(1.0);
            }
            Direction::Brake => {
                self.set_acceleration(1.0);
                self.set_braking(1.0);
            }
        }
    }
}
