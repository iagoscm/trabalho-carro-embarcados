mod i2c;
mod uart;
mod gpio;
mod car;
mod common;

use uart::modbus;
use car::control::CarControl;
use std::process::exit;
use ctrlc;
use std::sync::{Arc, Mutex};

fn main() {
    let mut meu_carro = CarControl::new();

    ctrlc::set_handler(move || {
      println!("\nCtrl+C pressionado! Finalizando o programa.");
      gpio::gpio::desliga();
      modbus::desliga();
      std::thread::sleep(std::time::Duration::from_millis(150));
      exit(0); 
    }).expect("\nErro ao configurar o handler de Ctrl+C");

    modbus::velocimetro();
    gpio::gpio::luz_motor();
    gpio::gpio::luz_freio();

    let parar_direita = Arc::new(Mutex::new(false));
    let parar_esquerda = Arc::new(Mutex::new(false));
      
    loop {
        //modbus::temp_motor();
        modbus::seta(&meu_carro, Arc::clone(&parar_direita), Arc::clone(&parar_esquerda));
        modbus::farol(&meu_carro);
        //modbus::cruise_control(&meu_carro);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

}
