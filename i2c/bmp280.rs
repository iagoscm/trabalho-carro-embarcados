use bmp280::{i2c::BMP280 as Device, Error, Measurements};
use linux_embedded_hal::{Delay, I2CError, I2cdev};

pub struct Bmp280 {
    delay: Delay,
    device: Device<I2cdev>,
}

impl Bmp280 {
    pub fn new() -> Self {
        let i2c = I2cdev::new("/dev/i2c-1").unwrap();
        let mut device = Device::new_primary(i2c);
        let mut delay = Delay;
        device.init(&mut delay).unwrap();

        Bmp280 { delay, device }
    }

    pub fn measure(&mut self) -> Result<Measurements<I2CError>, Error<I2CError>> {
        self.device.measure(&mut self.delay)
    }
}
