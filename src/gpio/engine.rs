use crate::common::{Direction, Car};
use rppal::gpio::{Gpio, OutputPin};

pub struct EngineControl {
    motor_pot: OutputPin,   
    brake_int: OutputPin, 
}

impl EngineControl {
    pub fn new(car: Car) -> Self {
        let gpio = Gpio::new().unwrap();

        EngineControl {
            motor_pot: gpio.get(23).unwrap().into_output_low(),
            brake_int: gpio.get(24).unwrap().into_output_low(),
        }
    }

    pub fn set_potency(&mut self, duty_cycle: f64) {
        self.motor_pot.set_pwm_frequency(1000.0, duty_cycle).unwrap();
    }

    pub fn set_brake_intensity(&mut self, duty_cycle: f64) {
        self.brake_int.set_pwm_frequency(1000.0, duty_cycle).unwrap();
    }

    pub fn set_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Accelerate => {
                self.set_potency(1.0);
                self.set_brake_intensity(0.0);
            }
            Direction::Brake => {
                self.set_potency(0.0);
                self.set_brake_intensity(1.0);
            }
            Direction::Stop => {
                self.set_potency(0.0);
                self.set_brake_intensity(0.0);
            }
        }
    }
}
