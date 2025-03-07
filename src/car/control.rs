use crate::common::{Direction, Car};
use crate::car::engine::EngineControl;
use std::sync::{Arc, Mutex};
use rppal::gpio::{OutputPin, InputPin};
use crate::gpio::pid::PidController;
use crate::uart::modbus;
use std::time::Duration;
use std::thread::sleep;

pub struct CarState {
    pub engine_control: EngineControl,
    pub current_speed: f32,
    pub current_rpm: f32,
    pub distance: f32,
    pub current_direction: Direction,
    pub seta_esquerda: bool,  
    pub seta_direita: bool,  
    pub farol_baixo: bool,   
    pub farol_alto: bool,
    pub cruise_control: bool,
    pub temp_alert: bool,
}

#[derive(Clone)]
pub struct CarControl {
    car: Arc<Mutex<CarState>>,
}

pub enum CruiseControl {
    Res,
    Cancel,
    Plus,
    Minus,
}

impl CarControl {
    pub fn new() -> Self {
        let engine_control = EngineControl::new(Car);
        let car_state = CarState {
            engine_control,
            current_speed: 0.0,
            current_rpm: 0.0,
            distance: 0.0,
            current_direction: Direction::Idle,
            seta_esquerda: false,
            seta_direita: false,
            farol_baixo: false,
            farol_alto: false,
            cruise_control: false,
	    temp_alert: false,
        };

        Self {
            car: Arc::new(Mutex::new(car_state)),
        }
    }

    pub fn get_car_state(&self) -> std::sync::MutexGuard<CarState>{
        self.car.lock().unwrap()
    }

    pub fn accelerate(&self, target_speed: f32) {
        let mut car = self.car.lock().unwrap();
        car.current_direction = Direction::Accelerate;
        car.engine_control.set_direction(Direction::Accelerate);
        car.current_speed = target_speed;
        car.current_rpm = target_speed*8.42;
        car.distance = car.distance + target_speed;	
        modbus::velocimetro(target_speed);
    }

    pub fn brake(&self, target_speed: f32) {
        let mut car = self.car.lock().unwrap();
        car.current_direction = Direction::Brake;
        car.engine_control.set_direction(Direction::Brake);
        car.current_speed = target_speed;
	    car.current_rpm = target_speed*8.42;	
        modbus::velocimetro(target_speed);
    }

    pub fn reverse(&self,  target_speed: f32) {
        let mut car = self.car.lock().unwrap();
        car.current_direction = Direction::Reverse;
        car.engine_control.set_direction(Direction::Reverse);
        car.current_speed = target_speed;
        car.current_rpm = target_speed*8.42;	
        car.distance = car.distance + target_speed;	
        modbus::velocimetro(target_speed);
    }

    pub fn idle(&self) {
        let mut car = self.car.lock().unwrap();
        car.current_direction = Direction::Idle;
        car.engine_control.set_direction(Direction::Idle);
    }

    pub fn off(&self) {
        let mut car = self.car.lock().unwrap();
        let mut engine = &mut car.engine_control;
        engine.motor_pot_pin.set_low();
        engine.brake_int_pin.set_low();
    }
}
