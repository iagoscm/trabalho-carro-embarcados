use embedded_graphics::{
    mono_font::{ascii, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle, Triangle},
    text::Text,
};

use rppal::i2c::I2c;

use ssd1306::{mode::BufferedGraphicsMode, 
    prelude::*, I2CDisplayInterface, Ssd1306};
use std::thread;
use std::time::Duration;
use i2cdev::linux::LinuxI2CDevice;
use i2cdev::core::I2CDevice;
use crate::i2c::bmp280::read_bmp280;
use crate::car::control::CarControl;

pub const SENSOR_ADDR: u16 = 0x3C;

struct CarState {
    speed: f32,
    rpm: f32,
    distance: f32,
    temperature: f32,
    temp_alert: bool,
    cruise_control: bool,
}

pub struct SSD1306 {
    display: Ssd1306<I2CInterface<I2c>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>,
    car: CarState,
    i2cdev: LinuxI2CDevice,
}

const i2c_device: &str = "/dev/i2c-1";
const device_address: u16 = 0x76;

impl SSD1306 {
    pub fn new(carro: &CarControl) -> Self {
        let i2c = I2c::new().unwrap();

        let i2cdev = LinuxI2CDevice::new("/dev/i2c-1", SENSOR_ADDR).unwrap();

        let temp = read_bmp280(i2c_device, device_address);

        let mut ssd1306 = Self {
            display: Ssd1306::new(
                I2CDisplayInterface::new(i2c),
                DisplaySize128x64,
                DisplayRotation::Rotate0,
            )
            .into_buffered_graphics_mode(),
            car: CarState {
                speed: 1000.0,
                rpm: 0.0,
                distance: 0.0,
                temperature: match temp {
                    Ok(temperature) => temperature, 
                    Err(err) => {
                        eprintln!("Erro ao ler sensor: {}", err); 
                        -1.0 
                    }
                },
                temp_alert: false,
                cruise_control: false,
            },
            i2cdev,
        };
        ssd1306.display.init().unwrap();
        ssd1306.refresh_screen(&carro);

        ssd1306
    }

    pub fn refresh_screen(&mut self, carro: &CarControl) {
        self.display.clear(BinaryColor::Off).unwrap();
        self.render_background();
	    self.update_info(&carro);
        self.render_info();
        self.display.flush().unwrap();
    }

    fn render_background(&mut self) {

        let text_style = MonoTextStyleBuilder::new()
            .font(&ascii::FONT_5X8)
            .text_color(BinaryColor::On)
            .build();

        Text::new("Carro da Alexia e do Iago", Point::new(0,5),text_style)
            .draw(&mut self.display)
            .unwrap();
        
    }

    fn update_info(&mut self, carro: &CarControl){
        let mut car_state = carro.get_car_state();

        let temp = read_bmp280(i2c_device, device_address);
        self.car.temperature = temp.expect("REASON");
        self.car.rpm = car_state.current_rpm;
        self.car.speed = car_state.current_speed;
        self.car.distance = car_state.distance;
        self.car.temp_alert = car_state.temp_alert;
        self.car.cruise_control = car_state.cruise_control;
    }

    fn render_info(&mut self) {
        
        let text_style = MonoTextStyleBuilder::new()
            .font(&ascii::FONT_6X10)
            .text_color(BinaryColor::On)
            .build();
        
        // A ser substituido com funções de update 
	    let vel_text = format!("Velocidade: {:.1}km/h", self.car.speed);
        let temp_text = format!("{:.1}°C", self.car.temperature);
        let rpm_text = format!("RPM: {:.1}", self.car.rpm);
        let distance_text = format!("Distância: {:.1} km", self.car.distance);
        let temp_alert_text = if self.car.temp_alert {
                "Alerta de Temperatura"
            } else {
                "Temperatura Normal"
            };
        let cruise_control_text = if self.car.cruise_control {
                "Cruise: ON"
            } else {
                "Cruise: OFF"
            };

        //Sequencia de prints na tela OLED
        Text::new(&vel_text, Point::new(0, 15), text_style)
            .draw(&mut self.display)
            .unwrap();
                
        Text::new(&temp_text, Point::new(0, 22), text_style)
            .draw(&mut self.display)
            .unwrap();
            
        Text::new(&rpm_text, Point::new(0, 30), text_style)
            .draw(&mut self.display)
            .unwrap();

        Text::new(&distance_text, Point::new(0, 38), text_style)
            .draw(&mut self.display)
            .unwrap();

        Text::new(&temp_alert_text, Point::new(0, 46), text_style)
            .draw(&mut self.display)
            .unwrap();

        Text::new(&cruise_control_text, Point::new(0, 54), text_style)
            .draw(&mut self.display)
            .unwrap();

    }

}
