mod i2c;
mod uart;
mod gpio;

//use std::thread;
//use uart::modbus::seta_esquerda;
//use uart::modbus::temp_motor;
//use std::time::Duration;

fn main() {
  // a cada 50ms pedir temperatura do motor, e ler comando das setas
  //loop {
    //temp_motor();
    //seta_esquerda();
    //thread::sleep(Duration::from_millis(50));
  //}
  gpio::pisca();
}