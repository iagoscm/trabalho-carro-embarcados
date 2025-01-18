use crate::common::{Direction, Car};
use crate::car::engine::EngineControl;
use std::sync::{Arc, Mutex};

pub struct CarState {
    pub engine_control: EngineControl,
    pub current_speed: f32,
    pub current_direction: Direction,
    pub seta_esquerda: bool,  
    pub seta_direita: bool,  
    pub farol_baixo: bool,   
    pub farol_alto: bool,
}

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
            current_direction: Direction::Idle,
            seta_esquerda: false,
            seta_direita: false,
            farol_baixo: false,
            farol_alto: false,
        };

        Self {
            car: Arc::new(Mutex::new(car_state)),
        }
    }

    pub fn get_car_state(&self) -> std::sync::MutexGuard<CarState>{
        self.car.lock().unwrap()
    }

    pub fn accelerate(&self) {
        let mut car = self.car.lock().unwrap();
        car.current_direction = Direction::Accelerate;
        car.engine_control.set_direction(Direction::Accelerate);
    }

    pub fn brake(&self) {
        let mut car = self.car.lock().unwrap();
        car.current_direction = Direction::Brake;
        car.engine_control.set_direction(Direction::Brake);
    }

    pub fn reverse(&self) {
        let mut car = self.car.lock().unwrap();
        car.current_direction = Direction::Reverse;
        car.engine_control.set_direction(Direction::Reverse);
    }

    pub fn idle(&self) {
        let mut car = self.car.lock().unwrap();
        car.current_direction = Direction::Idle;
        car.engine_control.set_direction(Direction::Idle);
    }
}
