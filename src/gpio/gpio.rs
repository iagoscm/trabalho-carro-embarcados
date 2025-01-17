use rppal::gpio::Gpio;
use std::thread;
use std::time::Duration;

// #define Motor_DIR1         RPI_V2_GPIO_P1_11 // BCM 17 
// #define Motor_DIR2         RPI_V2_GPIO_P1_12 // BCM 18
// #define Motor_POT          RPI_V2_GPIO_P1_16 // BCM 23 
// #define Freio_INT          RPI_V2_GPIO_P1_18 // BCM 24 
// #define Pedal_AC           RPI_V2_GPIO_P1_13 // BCM 27
// #define Pedal_FR           RPI_V2_GPIO_P1_15 // BCM 22 
// #define Sensor_hall_motor  RPI_V2_GPIO_P1_23 // BCM 11
// #define Sensor_hall_roda_A RPI_V2_GPIO_P1_29 // BCM 5 
// #define Sensor_hall_roda_B RPI_V2_GPIO_P1_31 // BCM 6 
// #define Farol_Baixo        RPI_V2_GPIO_P1_35 // BCM 19 
// #define Farol_Alto         RPI_V2_GPIO_P1_37 // BCM 26 
// #define Luz_Freio          RPI_V2_GPIO_P1_22 // BCM 25 
// #define Luz_Seta_Esq       RPI_V2_GPIO_P1_24 // BCM 8 
// #define Luz_Seta_Dir       RPI_V2_GPIO_P1_26 // BCM 7 
// #define Luz_Temp_Motor     RPI_V2_GPIO_P1_32 // BCM 12 

pub mod gpio {
  pub const MOTOR_DIR1: u8 = 17;
  pub const MOTOR_DIR2: u8 = 18;
  pub const MOTOR_POT: u8 = 23;
  pub const FREIO_INT: u8 = 24;
  pub const PEDAL_AC: u8 = 27;
  pub const PEDAL_FR: u8 = 22;
  pub const SENSOR_HALL_MOTOR: u8 = 11;
  pub const SENSOR_HALL_RODA_A: u8 = 5;
  pub const SENSOR_HALL_RODA_B: u8 = 6;
  pub const FAROL_BAIXO: u8 = 19;
  pub const FAROL_ALTO: u8 = 26;
  pub const LUZ_FREIO: u8 = 25;
  pub const LUZ_SETA_ESQ: u8 = 8;
  pub const LUZ_SETA_DIR: u8 = 7;
  pub const LUZ_TEMP_MOTOR: u8 = 12;
}

pub mod constants {
  pub const RODA_DIAMETRO: f32 = 63.0;
  pub const MOTOR_TEMP_LIMITE: f32 = 115.0;
  pub const CRUISE_CONTROL_PASSO: f32 = 1.0;
}

pub fn pisca_seta_esquerda() {
  let gpio = Gpio::new()
      .expect("Erro ao configurar GPIO, o programa está sendo executado em uma raspberry pi?");

  let mut pin = gpio
      .get(gpio::LUZ_SETA_ESQ)
      .expect(format!("Erro ao obter pino {}, talvez esteja ocupado.", gpio::LUZ_SETA_ESQ).as_str())
      .into_output_low();

  loop {
      pin.set_high();
      thread::sleep(Duration::from_millis(500));

      pin.set_low();
      thread::sleep(Duration::from_millis(500));
  }
}

pub fn pisca_seta_direita() {
  let gpio = Gpio::new()
      .expect("Erro ao configurar GPIO, o programa está sendo executado em uma raspberry pi?");

  let mut pin = gpio   // obtém o pino 17 e o configura como saída em nível baixo
      .get(gpio::LUZ_SETA_DIR)
      .expect(format!("Erro ao obter pino {}, talvez esteja ocupado.", gpio::LUZ_SETA_ESQ).as_str())
      .into_output_low();

  loop {
      pin.set_high();
      thread::sleep(Duration::from_millis(500));

      pin.set_low();
      thread::sleep(Duration::from_millis(500));
  }
}

pub fn farol_baixo_desliga() {

  let gpio = Gpio::new()
    .expect("Erro ao configurar GPIO, o programa está sendo executado em uma raspberry pi?");

  let mut pin = gpio 
    .get(gpio::FAROL_BAIXO)
    .expect(format!("Erro ao obter pino {}, talvez esteja ocupado.", gpio::FAROL_BAIXO).as_str())
    .into_output_low();

    pin.set_low();
}

pub fn farol_alto_desliga() {

  let gpio = Gpio::new()
    .expect("Erro ao configurar GPIO, o programa está sendo executado em uma raspberry pi?");

  let mut pin = gpio   
    .get(gpio::FAROL_ALTO)
    .expect(format!("Erro ao obter pino {}, talvez esteja ocupado.", gpio::FAROL_ALTO).as_str())
    .into_output_low();

  pin.set_low();
}

pub fn farol_baixo_liga() {

  let gpio = Gpio::new()
    .expect("Erro ao configurar GPIO, o programa está sendo executado em uma raspberry pi?");

  let mut pin = gpio
    .get(gpio::FAROL_BAIXO)
    .expect(format!("Erro ao obter pino {}, talvez esteja ocupado.", gpio::FAROL_BAIXO).as_str())
    .into_output_low();

  pin.set_high();
}

pub fn farol_alto_liga() {

  let gpio = Gpio::new()
    .expect("Erro ao configurar GPIO, o programa está sendo executado em uma raspberry pi?");

  let mut pin = gpio   
    .get(gpio::FAROL_ALTO)
    .expect(format!("Erro ao obter pino {}, talvez esteja ocupado.", gpio::LUZ_SETA_ESQ).as_str())
    .into_output_low();

  pin.set_high();
}


pub fn desliga() {
  let gpio = Gpio::new()
    .expect("Erro ao configurar GPIO, o programa está sendo executado em uma raspberry pi?");

  let mut farol_alto = gpio   
    .get(gpio::FAROL_ALTO)
    .expect(format!("Erro ao obter pino {}, talvez esteja ocupado.", gpio::FAROL_ALTO).as_str())
    .into_output_low();

  let mut farol_baixo = gpio   
    .get(gpio::FAROL_BAIXO)
    .expect(format!("Erro ao obter pino {}, talvez esteja ocupado.", gpio::FAROL_BAIXO).as_str())
    .into_output_low();

  let mut seta_esquerda = gpio   
    .get(gpio::LUZ_SETA_ESQ)
    .expect(format!("Erro ao obter pino {}, talvez esteja ocupado.", gpio::LUZ_SETA_ESQ).as_str())
    .into_output_low();

  let mut seta_direita = gpio   
    .get(gpio::LUZ_SETA_DIR)
    .expect(format!("Erro ao obter pino {}, talvez esteja ocupado.", gpio::LUZ_SETA_DIR).as_str())
    .into_output_low();

    let mut temp_motor = gpio   
    .get(gpio::LUZ_TEMP_MOTOR)
    .expect(format!("Erro ao obter pino {}, talvez esteja ocupado.", gpio::LUZ_TEMP_MOTOR).as_str())
    .into_output_low();

    let mut freio = gpio   
    .get(gpio::LUZ_FREIO)
    .expect(format!("Erro ao obter pino {}, talvez esteja ocupado.", gpio::LUZ_FREIO).as_str())
    .into_output_low();
}

pub fn luz_motor(estado: bool) {
  let gpio = Gpio::new()
      .expect("Erro ao configurar GPIO, o programa está sendo executado em uma raspberry pi?");

  let mut pin = gpio
      .get(gpio::LUZ_TEMP_MOTOR)
      .expect(format!("Erro ao obter pino {}, talvez esteja ocupado.", gpio::LUZ_TEMP_MOTOR).as_str())
      .into_output_low();

  if estado == true { pin.set_high(); }
}

pub fn luz_freio(estado: bool) {
  let gpio = Gpio::new()
      .expect("Erro ao configurar GPIO, o programa está sendo executado em uma raspberry pi?");

  let mut pin = gpio
      .get(gpio::LUZ_FREIO)
      .expect(format!("Erro ao obter pino {}, talvez esteja ocupado.", gpio::LUZ_FREIO).as_str())
      .into_output_low();

  if estado == true { pin.set_high(); }
}
