mod i2c;
mod uart;
mod gpio;
mod car;
mod common;

use uart::modbus;
use car::control::CarControl;
use uart::modbus::CruiseControlState;
use std::process::exit;
use ctrlc;
use std::sync::{Arc, Mutex};
use i2c::ssd1306::SSD1306;
use i2c::bmp280::read_bmp280;



fn main() {
    let mut meu_carro = CarControl::new();
    let mut cruise_state = CruiseControlState {
	    is_active: false,
	    debounce: false,
    };

    let i2c_device = "/dev/i2c-1";
    let device_address = 0x76;

    let mut tela_led = SSD1306::new();

    ctrlc::set_handler(move || {
      println!("\nCtrl+C pressionado! Finalizando o programa.");
      gpio::gpio::desliga();
      modbus::desliga();
      std::thread::sleep(std::time::Duration::from_millis(150));
      exit(0); 
    }).expect("\nErro ao configurar o handler de Ctrl+C");

    modbus::velocimetro(0.0);
    gpio::gpio::luz_motor();
    gpio::gpio::luz_freio();

    let parar_direita = Arc::new(Mutex::new(false));
    let parar_esquerda = Arc::new(Mutex::new(false));      

    loop {
        //modbus::temp_motor();
	      //let mut temp_int = read_bmp280(i2c_device, device_address);
	      SSD1306::refresh_screen(&mut tela_led);
        modbus::seta(&meu_carro, Arc::clone(&parar_direita), Arc::clone(&parar_esquerda));
        modbus::farol(&meu_carro);
        //modbus::cruise_control(&meu_carro, &mut cruise_state);
        std::thread::sleep(std::time::Duration::from_millis(50));
	
    }

}
