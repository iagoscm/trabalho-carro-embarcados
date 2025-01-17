const KP: f64 = 0.5;
const KI: f64 = 0.05;
const KD: f64 = 40.0;
const MAX_OUTPUT: f64 = 100.0;
const MIN_OUTPUT: f64 = 0.0;
const T: f64 = 1.0;

pub struct PidController {
    integral: f64,
    previous_error: f64,
}

impl PidController {
    pub fn new() -> Self {
        PidController {
            integral: 0.0,
            previous_error: 0.0,
        }
    }

    pub fn compute(&mut self, setpoint: f64, measured: f64) -> f64 {
        let error = setpoint - measured;

        let proportional = KP * error;

        self.integral += error * T;
        let integral = KI * self.integral;

        let derivative = KD * (error - self.previous_error) / T;

        self.previous_error = error;

        let mut output = proportional + integral + derivative;

        if output > MAX_OUTPUT {
            output = MAX_OUTPUT;
        } else if output < MIN_OUTPUT {
            output = MIN_OUTPUT;
        }

        output
    }
}
