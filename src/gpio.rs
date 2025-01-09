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

const MOTOR_DIR1: u8 = 17;
const MOTOR_DIR2: u8 = 18;
const MOTOR_POT: u8 = 23;
const FREIO_INT: u8 = 24;
const PEDAL_AC: u8 = 27;
const PEDAL_FR: u8 = 22;
const SENSOR_HALL_MOTOR: u8 = 11;
const SENSOR_HALL_RODA_A: u8 = 5;
const SENSOR_HALL_RODA_B: u8 = 6;
const FAROL_BAIXO: u8 = 19;
const FAROL_ALTO: u8 = 26;
const LUZ_FREIO: u8 = 25;
const LUZ_SETA_ESQ: u8 = 8;
const LUZ_SETA_DIR: u8 = 7;
const LUZ_TEMP_MOTOR: u8 = 12;

pub fn pisca() {
  // Inicializa a instância de GPIO, isso realiza algumas checagens de permissão
  let gpio = Gpio::new()
      .expect("Erro ao configurar GPIO, o programa está sendo executado em uma raspberry pi?");

  // Obtém o pino 17 e o configura como saída em nível baixo, pode falhar se o pino estiver ocupado
  let mut pin = gpio
      .get(LUZ_SETA_ESQ)
      .expect(format!("Erro ao obter pino {}, talvez esteja ocupado.", LUZ_SETA_ESQ).as_str())
      .into_output_low();

  loop {
      pin.set_high();
      thread::sleep(Duration::from_millis(500));

      pin.set_low();
      thread::sleep(Duration::from_millis(500));
  }
}
