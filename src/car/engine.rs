use crate::common::{Direction, Car};
use crate::gpio::gpio::gpio::{MOTOR_POT, FREIO_INT, SENSOR_HALL_MOTOR, SENSOR_HALL_RODA_A, SENSOR_HALL_RODA_B};
use rppal::gpio::{Gpio, OutputPin, InputPin};
use std::time::{Instant, Duration};

pub struct EngineControl {
    pub motor_pot_pin: OutputPin,
    pub brake_int_pin: OutputPin,
    pub motor_hall: InputPin,
    pub roda_a_hall: InputPin,
    pub roda_b_hall: InputPin,
    pulse_count: u32,
    last_pulse_time: Instant,
}

impl EngineControl {
    pub fn new(car: Car) -> Self {
        let gpio_lib = Gpio::new().unwrap();

        EngineControl {
            motor_pot_pin: gpio_lib.get(MOTOR_POT).unwrap().into_output_low(),
            brake_int_pin: gpio_lib.get(FREIO_INT).unwrap().into_output_low(),
            motor_hall: gpio_lib.get(SENSOR_HALL_MOTOR).unwrap().into_input(),
            roda_a_hall: gpio_lib.get(SENSOR_HALL_RODA_A).unwrap().into_input(),
            roda_b_hall: gpio_lib.get(SENSOR_HALL_RODA_B).unwrap().into_input(),
            pulse_count: 0,
            last_pulse_time: Instant::now(),
        }
    }

    pub fn set_acceleration(&mut self, duty_cycle: f64) {
    	self.motor_pot_pin.set_pwm_frequency(1000.0, duty_cycle / 100.0).unwrap();
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
		println!("Acelerando!");
            }
            Direction::Reverse => {
                self.set_acceleration(0.0);
                self.set_braking(1.0);
		println!("Dando ré!");
            }
            Direction::Brake => {
                self.set_acceleration(1.0);
                self.set_braking(1.0);
		println!("Freiando!");
            }
        }
    }

    pub fn calculate_speed(&self) -> f32 {
        let pulse_interval = self.last_pulse_time.elapsed();
        
        if pulse_interval > Duration::from_secs(1) {
            return 0.0;
        }

        let wheel_diameter = 0.63;
        let pi = std::f32::consts::PI;
        let wheel_circumference = wheel_diameter * pi;
        let pulses_per_revolution = 1.0; 
        let time_per_pulse = pulse_interval.as_secs_f32(); 

        let speed_mps = wheel_circumference / time_per_pulse;
    
        // Converte para km/h
        speed_mps * 3.6
    }

    pub fn count_pulses(&mut self) {
        if self.roda_a_hall.is_high() {
            self.pulse_count += 1;
            self.last_pulse_time = Instant::now();
        }
    }

    pub fn update(&mut self) {
        self.count_pulses();
        let speed = self.calculate_speed();
        println!("{} km/h", speed);
    }
}
