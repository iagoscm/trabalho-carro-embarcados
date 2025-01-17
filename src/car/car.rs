use crate::common::{Direction, Car};
use crate::engine_control::EngineControl;
use std::sync::{Arc, Mutex};
use stoppable_thread::StoppableHandle;

pub struct CarState {
    pub car: Car,
    pub engine_control: EngineControl,
    pub current_direction: Direction,
}

pub struct CarControl {
    car_state: Arc<Mutex<CarState>>,
    control_thread: Option<StoppableHandle<()>>,
}

impl CarControl {
    pub fn new() -> Self {
        let car = Car;
        let engine_control = EngineControl::new(car);

        let car_state = CarState {
            car,
            engine_control,
            current_direction: Direction::Stop,
        };

        Self {
            car_state: Arc::new(Mutex::new(car_state)),
            control_thread: None,
        }
    }

    pub fn start(&mut self) {
        let car_state = self.car_state.clone();

        self.control_thread = Some(stoppable_thread::spawn(move |stopped| {
            while !stopped.get() {
                let mut state = car_state.lock().unwrap();
                state.engine_control.set_direction(state.current_direction);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }));
    }

    pub fn set_direction(&self, direction: Direction) {
        let mut state = self.car_state.lock().unwrap();
        state.current_direction = direction;
    }

    pub fn stop(&mut self) {
        if let Some(handle) = self.control_thread.take() {
            handle.stop().unwrap();
        }
    }
}
