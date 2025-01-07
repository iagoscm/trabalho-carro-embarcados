use crate::uart::esp32::Car;
use std::{
    fs::read_to_string,
    io::{Error, ErrorKind::InvalidData},
}

const TEMPERATURE_FILE_1: &str = "/sys/bus/iio/devices/iio:device0/in_temp_input";
const TEMPERATURE_FILE_2: &str = "/sys/bus/iio/devices/iio:device1/in_temp_input";

pub fn measure_temperature(car: Car) -> Result<f32, Error> {
    let file = match car {
        Car::One => TEMPERATURE_FILE_1,
        Car::Two => TEMPERATURE_FILE_2,
};

 let in_temp_input: f32 = read_to_string(file)?
        .parse()
        .map_err(|e| Error::new(InvalidData, e))?;

Ok((((in_temp_input / 1000.0) * 100.0) + 0.5) / 100.0)

}
