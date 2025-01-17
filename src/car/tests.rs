use crate::car::control::CarControl;
use crate::common::Direction;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acceleration() {
        let car_control = CarControl::new();
        car_control.accelerate();
        let car = car_control.get_car_state();
        assert_eq!(car.current_direction, Direction::Accelerate);
    }

    #[test]
    fn test_braking() {
        let car_control = CarControl::new();
        car_control.brake();
        let car = car_control.get_car_state();
        assert_eq!(car.current_direction, Direction::Brake);
    }

    #[test]
    fn test_idle() {
        let car_control = CarControl::new();
        car_control.idle();
        let car = car_control.get_car_state();
        assert_eq!(car.current_direction, Direction::Idle);
    }

    #[test]
    fn test_reverse() {
        let car_control = CarControl::new();
        car_control.reverse();
        let car = car_control.get_car_state();
        assert_eq!(car.current_direction, Direction::Reverse);
    }
}
