mod tests {
    use crate::gpio::pid::PidController;

    #[test]
    fn test_pid_controller() {
        let mut pid = PidController::new();
        let setpoint = 60.0;
        let measured = 50.0;

        let control_signal = pid.compute(setpoint, measured);

        assert!(control_signal > 0.0); 
    }
}
