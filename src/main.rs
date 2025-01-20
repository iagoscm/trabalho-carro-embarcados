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
use i2c::ssd1306::SSD1306;
use i2c::bmp280::read_bmp280;

fn main() {

  gpio::gpio::desliga();
  modbus::desliga();

  let mut meu_carro = CarControl::new();

  let i2c_device = "/dev/i2c-1";
  let device_address = 0x76;

  let mut tela_led = SSD1306::new(&meu_carro);

  let parar_loop = Arc::new(Mutex::new(false));

  let parar_loop_ctrlc = Arc::clone(&parar_loop);

  ctrlc::set_handler(move || {
    println!("\nCtrl+C pressionado! Finalizando o programa.");
    *parar_loop_ctrlc.lock().unwrap() = true;
  }).expect("\nErro ao configurar o handler de Ctrl+C");
  println!("---------------------------------------");
  println!("Bem vindo ao Carro da Alexia e do Iago!");
  println!("---------------------------------------");

  println!("----------------------------------------------------------------------------------------------");
  println!("- Uart e GPIOs estão completamente implementados.");
  println!("- PID implementado, PWMs sendo atualizados, porém aceleração incompleta, \ntendo somente atualizações estáticas.");
  println!("- O painel está com seu retorno devido além do gráfico de Vel/RPM; Luz de temperatura \nnão foi vista sendo ultrapassada o limite mas a função está devidamente feita");
  println!("- I2c sendo devidamente usado com o BMP280 e print das variáveis atualizadas no OLED SSD");
  println!("- Código modularizado na medida do possível; Códigos da UART e da GPIO precisam de refatoração");
  println!("----------------------------------------------------------------------------------------------");

  std::thread::sleep(std::time::Duration::from_millis(1000));

  let parar_direita = Arc::new(Mutex::new(false));
  let parar_esquerda = Arc::new(Mutex::new(false));      

  modbus::velocimetro(0.0);

  loop {
    if *parar_loop.lock().unwrap(){
      break;
    }
	
	  let mut temp_int = read_bmp280(i2c_device, device_address);
	  SSD1306::refresh_screen(&mut tela_led, &meu_carro);
    modbus::seta(&meu_carro, Arc::clone(&parar_direita), Arc::clone(&parar_esquerda));
    modbus::farol(&meu_carro);
	  modbus::temp_motor(&meu_carro);
    gpio::gpio::pedal(&mut meu_carro);
    modbus::cruise_control(&meu_carro);
    std::thread::sleep(std::time::Duration::from_millis(50));
	
  }

  modbus::desliga();
  gpio::gpio::desliga();

  println!("---------------------------------");
  println!("Obrigado pela atenção e pelo uso!");
  println!("---------------------------------");

    exit(0);

}
