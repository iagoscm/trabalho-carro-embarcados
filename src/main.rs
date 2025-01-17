mod i2c;
mod uart;
mod gpio;
mod car;
mod common;

use uart::modbus;
use car::control::CarControl;
use std::process::exit;
use ctrlc;

fn main() {
    let mut meu_carro = CarControl::new();

    ctrlc::set_handler(move || {
      println!("\nCtrl+C pressionado! Finalizando o programa.");
      gpio::gpio::desliga();
      modbus::desliga();
      exit(0); 
    }).expect("\nErro ao configurar o handler de Ctrl+C");

    loop {
        //modbus::temp_motor();
        modbus::seta(&meu_carro);
        modbus::farol(&meu_carro);
        //modbus::cruise_control(&meu_carro);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

}
