#[derive(PartialEq, Debug)]
pub enum Direction {
    Idle,
    Accelerate,
    Reverse,
    Brake,
}

pub struct Car;
