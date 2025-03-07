use std::thread::sleep;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::thread;

use rppal::uart::Queue;
use rppal::uart::{Parity, Uart};

use crate::uart::crc;
use crate::gpio::gpio;
use crate::car::control::{ CarControl, CruiseControl, CarState };
use crate::common::{Direction, Car};

const BAUD_RATE: u32 = 115200;
const DATA_BITS: u8 = 8;
const STOP_BITS: u8 = 1;

const SOURCE_ADDRESS: u8 = 0x00;
const TARGET_ADDRESS: u8 = 0x01;
const REGISTER_CODE: [u8; 4] = [3, 7, 4, 3];


pub const LE_TEMP: ModbusOperation = ModbusOperation {
    code: 0x23,
    subcode: 0xAA,
    qtd: None,
};

pub const CONTROL_READ_SETA: ModbusOperation = ModbusOperation {
    code: 0x03,
    subcode: 0x00,
    qtd: Some(1),
};

pub const CONTROL_WRITE_SETA: ModbusOperation = ModbusOperation {
    code: 0x06,
    subcode: 0x00,
    qtd: Some(1),
};

pub const CONTROL_READ_SETA_ESQUERDA: ModbusOperation = ModbusOperation {
    code: 0x03,
    subcode: 0x0B,
    qtd: Some(1),
};

pub const CONTROL_READ_SETA_DIREITA: ModbusOperation = ModbusOperation {
    code: 0x03,
    subcode: 0x0C,
    qtd: Some(1),
};

pub const CONTROL_WRITE_SETA_ESQUERDA: ModbusOperation = ModbusOperation {
    code: 0x06,
    subcode: 0x0B,
    qtd: Some(1),
};

pub const CONTROL_WRITE_SETA_DIREITA: ModbusOperation = ModbusOperation {
    code: 0x06,
    subcode: 0x0C,
    qtd: Some(1),
};

pub const CONTROL_CRUISE_READ: ModbusOperation = ModbusOperation {
    code: 0x03,
    subcode: 0x01,  
    qtd: Some(1),
};

pub const CONTROL_CRUISE_WRITE: ModbusOperation = ModbusOperation {
    code: 0x06,
    subcode: 0x01,  
    qtd: Some(1),
};

pub const CONTROL_VELOCIDADE_WRITE: ModbusOperation = ModbusOperation {
    code: 0x06,
    subcode: 0x03,  
    qtd: Some(4),
};

pub const CONTROL_VELOCIDADE_READ: ModbusOperation = ModbusOperation {
    code: 0x03,
    subcode: 0x03,  
    qtd: Some(4),
};


pub const CONTROL_RPM_WRITE: ModbusOperation = ModbusOperation {
    code: 0x06,
    subcode: 0x07,  
    qtd: Some(4),
};

pub const CONTROL_RPM_READ: ModbusOperation = ModbusOperation {
    code: 0x03,
    subcode: 0x07,  
    qtd: Some(4),
};


pub const CONTROL_FAROL: ModbusOperation = ModbusOperation {
    code: 0x03,
    subcode: 0x02,  
    qtd: Some(1),
};

pub const CONTROL_WRITE_FAROL: ModbusOperation = ModbusOperation {
    code: 0x06,
    subcode: 0x02,  
    qtd: Some(1),
};

pub const CONTROL_FAROL_ALTO: ModbusOperation = ModbusOperation {
    code: 0x03,
    subcode: 0x0D, // 13
    qtd: Some(1),
};

pub const CONTROL_FAROL_BAIXO: ModbusOperation = ModbusOperation {
    code: 0x03,
    subcode: 0x0E, // 14
    qtd: Some(1),
};

pub const CONTROL_WRITE_FAROL_ALTO: ModbusOperation = ModbusOperation {
    code: 0x06,
    subcode: 0x0D,  
    qtd: Some(1),
};

pub const CONTROL_WRITE_FAROL_BAIXO: ModbusOperation = ModbusOperation {
    code: 0x06,
    subcode: 0x0E,  
    qtd: Some(1),
};

#[allow(non_snake_case)]
pub fn READ_REGISTERS(address: u8, qtd: u8) -> ModbusOperation {
    ModbusOperation {
        code: 0x03,
        subcode: address,
        qtd: Some(qtd),
    }
}

#[allow(non_snake_case)]
pub fn WRITE_REGISTERS(address: u8, qtd: u8) -> ModbusOperation {
    ModbusOperation {
        code: 0x06,
        subcode: address,
        qtd: Some(qtd),
    }
}

pub fn create_modbus(operation: ModbusOperation, data: &[u8]) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(9 + data.len());

    buffer.push(TARGET_ADDRESS); 

    buffer.push(operation.code); 

    buffer.push(operation.subcode); 

    if let Some(qtd) = operation.qtd {
        buffer.push(qtd);
    }

    for byte in data {
        buffer.push(*byte);
    }

    for byte in REGISTER_CODE { 
        buffer.push(byte);
    }

    for byte in crc::hash(&buffer).to_le_bytes() {
        buffer.push(byte);
    }
    
    buffer
}

pub fn checa_crc16(buffer: &[u8]) -> bool {
    if buffer.len() < 2 {
        return false;
    }

    let received_crc = u16::from_le_bytes([buffer[buffer.len() - 2], buffer[buffer.len() - 1]]);
    let calculated_crc = crc::hash(&buffer[..buffer.len() - 2]);

    received_crc == calculated_crc
}

#[derive(Clone, Copy)]
pub struct ModbusOperation {
    code: u8,
    subcode: u8,
    qtd: Option<u8>,
}


pub fn temp_motor(carro: &CarControl) {
    let mut uarto = open_uart();
    let envia = create_modbus(LE_TEMP, &[]);
    let mut car_state = carro.get_car_state();

    uarto.write(envia.as_slice()).unwrap(); 
    sleep(Duration::from_millis(50));
    car_state.temp_alert = false;
    
    let mut leitura = vec![0; 7];
    let response = uarto.read(&mut leitura).unwrap();
    let float_value = f32::from_le_bytes(leitura[3..7].try_into().expect("Falha na conversão para f32"));

    if float_value >= 115.0 {
        gpio::luz_motor();
	    car_state.temp_alert = true;
    }

    drop(uarto);
}

pub fn seta(carro: &CarControl,parar_direita: Arc<Mutex<bool>>, parar_esquerda: Arc<Mutex<bool>>) {
    let mut uarto = open_uart();
    let mut envia = create_modbus(CONTROL_READ_SETA, &[]);
    let mut car_state = carro.get_car_state();

    uarto.write(&envia).unwrap(); 
    sleep(Duration::from_millis(50));

    let mut leitura = vec![0; 7];
    uarto.read(&mut leitura);

    let byte_da_seta = leitura[2];
    let ultimos_dois_bits = byte_da_seta & 0b00000011;
    //println!("Os dois últimos bits do byte da seta: {:02b}", ultimos_dois_bits);
    if ultimos_dois_bits == 0b10 {muda_seta(1, &mut car_state.seta_direita,parar_direita, parar_esquerda);} // direita
    else if ultimos_dois_bits == 0b01 {muda_seta(2, &mut car_state.seta_esquerda,parar_direita, parar_esquerda);} // esquerda
    
    drop(uarto);
}

fn muda_seta(direction: usize, estado: &mut bool,parar_direita: Arc<Mutex<bool>>, parar_esquerda: Arc<Mutex<bool>>) {
    let mut uarto = open_uart();
    let mut seta = create_modbus(CONTROL_WRITE_SETA, &[0]);
    uarto.write(&seta).unwrap();
    //println!("Direcao: {}",direction);

    if direction == 1 {
        println!("Seta Direita");
    
        if !*estado {
            *parar_direita.lock().unwrap() = false;
            let parar_clone = Arc::clone(&parar_direita);
            thread::spawn(move || {
                gpio::pisca_seta_direita(parar_clone);
            });
        } else {
            *parar_direita.lock().unwrap() = true;
        }

        let mut request = create_modbus(CONTROL_WRITE_SETA_DIREITA, &[if *estado { 0 } else { 1 }]);
        *estado = !*estado;
        let mut response = [0; 5];
        uarto.write(&request);
        sleep(Duration::from_millis(50));
        uarto.read(&mut response);

    } else {
        println!("Seta Esquerda");
        
        if !*estado {
            *parar_esquerda.lock().unwrap() = false;
            let parar_clone = Arc::clone(&parar_esquerda);
            thread::spawn(move || {
                gpio::pisca_seta_esquerda(parar_clone);
            });
        } else {
            *parar_esquerda.lock().unwrap() = true;
        }

        let mut request = create_modbus(CONTROL_WRITE_SETA_ESQUERDA, &[if *estado { 0 } else { 1 }]);
        *estado = !*estado;
        let mut response = [0; 5];
        uarto.write(&request);

        sleep(Duration::from_millis(50));
        uarto.read(&mut response);
    }
 
    
}

pub fn open_uart() -> Uart {
    let mut uarto = Uart::new(BAUD_RATE, Parity::None, DATA_BITS, STOP_BITS).unwrap();
    uarto 
}

pub fn farol(carro: &CarControl) {
    let mut uarto = open_uart();
    let mut envia = create_modbus(CONTROL_FAROL, &[]);
    let mut car_state = carro.get_car_state();

    uarto.write(&envia).unwrap(); 
    sleep(Duration::from_millis(50));

    let mut leitura = vec![0; 7];
    uarto.read(&mut leitura);

    let byte_do_farol = leitura[2];
    let ultimos_dois_bits = byte_do_farol & 0b00000011;
    // println!("Os dois últimos bits do byte do FAROL: {:02b}", ultimos_dois_bits);
    if ultimos_dois_bits == 0b10 {muda_farol(1, &mut car_state.farol_alto);} // farol alto
    else if ultimos_dois_bits == 0b01 {muda_farol(2, &mut car_state.farol_baixo);} // farol baixo
        
    drop(uarto);
}

pub fn muda_farol(farol_direcao: usize, estado: &mut bool){
    let mut uarto = open_uart();
    let mut farol = create_modbus(CONTROL_WRITE_FAROL, &[0]);
    uarto.write(&farol).unwrap();
    
    let mut success = false;
    if farol_direcao == 2 { 
        //println!("FAROL BAIXO | Estado: {}",*estado);
        if *estado == true { gpio::farol_baixo_desliga(); println!("Desliga farol baixo"); }
        else { gpio::farol_baixo_liga(); println!("Liga farol baixo"); }
        
        //println!("Estado FAROL BAIXO: {}",estado);
        let mut request = create_modbus(CONTROL_WRITE_FAROL_BAIXO, &[if *estado { 0 } else { 1 }]);
        *estado = !*estado;
        let mut response = [0; 5];
        uarto.write(&request);

        sleep(Duration::from_millis(50));
        uarto.read(&mut response);

    } else {
        //println!("FAROL ALTO");
        if *estado == true { gpio::farol_alto_desliga(); println!("Desliga farol alto"); }
        else { gpio::farol_alto_liga(); println!("Liga farol alto"); }
        //println!("Estado FAROL ALTO: {}",estado);
        let mut request = create_modbus(CONTROL_WRITE_FAROL_ALTO, &[if *estado { 0 } else { 1 }]);
        *estado = !*estado;
        let mut response = [0; 5];
        uarto.write(&request);
        sleep(Duration::from_millis(50));
        uarto.read(&mut response);
    }

    drop(uarto);
}

const RES: u8 = 0x01;
const CANCEL: u8 = 0x02;
const SET_PLUS: u8 = 0x04;
const SET_MINUS: u8 = 0x10;

pub fn cruise_control(carro: &CarControl) {
    let mut uarto = open_uart();
    let mut envia = create_modbus(CONTROL_CRUISE_READ, &[]);
    let mut car_state = carro.get_car_state();

    uarto.write(&envia).unwrap();
    sleep(Duration::from_millis(50));

    let mut leitura = vec![0; 7];
    uarto.read(&mut leitura);
    let byte_do_cruise = leitura[2];

    match byte_do_cruise {
        0b00000001 if !car_state.cruise_control => {
            println!("Cruise Control Ativado!");
            let mut request = create_modbus(CONTROL_CRUISE_WRITE, &[1]);
            let mut response = [0; 5];
            uarto.write(&request);
            car_state.cruise_control = true;
            car_state.current_direction = Direction::Idle;	
            sleep(Duration::from_millis(50));
            uarto.read(&mut response);
        },
        0b00000010 if car_state.cruise_control => {
            println!("Cruise Control Cancelado!");
            let mut request = create_modbus(CONTROL_CRUISE_WRITE, &[2]);
            let mut response = [0; 5];
            uarto.write(&request);
            car_state.cruise_control = false;
            sleep(Duration::from_millis(50));
            uarto.read(&mut response);
        },
        0b00000100 if car_state.cruise_control => {
            println!("Cruise Control - Aumentando velocidade!");
	        let mut request = create_modbus(CONTROL_CRUISE_WRITE, &[4]);	    
	        //car_state.cruise_control = false; 
	        if car_state.current_speed < 200.01 {
        	car_state.current_speed = car_state.current_speed + 1.0;
		    car_state.current_rpm = car_state.current_rpm + 8.42;
		    car_state.distance += 1.0;
            velocimetro(car_state.current_speed);
            }
	        println!("Ajustando para velocidade: {}", car_state.current_speed);
            sleep(Duration::from_millis(500));
        },
        0b00001000 if car_state.cruise_control => {
            println!("Cruise Control - Diminuindo velocidade!");
	        let mut request = create_modbus(CONTROL_CRUISE_WRITE, &[8]);
            //car_state.cruise_control = false;
            if car_state.current_speed > 0.0 {
            car_state.current_speed = car_state.current_speed - 1.0;
		    car_state.current_rpm = car_state.current_rpm - 8.42;
		    velocimetro(car_state.current_speed);
            }
	        println!("Ajustando para velocidade: {}", car_state.current_speed);
            sleep(Duration::from_millis(500));
        },
        _ => {},
    }

    drop(uarto);
}


pub fn desliga() {
    let mut uarto = open_uart();
    let mut request = create_modbus(CONTROL_WRITE_FAROL_ALTO, &[0]);
    uarto.write(&request).unwrap();
    let mut request = create_modbus(CONTROL_WRITE_FAROL_BAIXO, &[0]);
    uarto.write(&request).unwrap();
    let mut request = create_modbus(CONTROL_WRITE_SETA_ESQUERDA, &[0]);
    uarto.write(&request).unwrap();
    let mut request = create_modbus(CONTROL_WRITE_SETA_DIREITA, &[0]);
    uarto.write(&request).unwrap();
    let mut request = create_modbus(CONTROL_CRUISE_READ, &[0]);
    uarto.write(&request).unwrap();
    let mut request = create_modbus(CONTROL_CRUISE_WRITE, &[0]);
    uarto.write(&request).unwrap();
    let mut request = create_modbus(LE_TEMP, &[0]);
    uarto.write(&request).unwrap();
    let mut request = create_modbus(CONTROL_READ_SETA, &[0]);
    uarto.write(&request).unwrap();
    let mut request = create_modbus(CONTROL_WRITE_SETA, &[0]);
    uarto.write(&request).unwrap();
    let mut request = create_modbus(CONTROL_FAROL, &[0]);
    uarto.write(&request).unwrap();
    let mut request = create_modbus(CONTROL_WRITE_FAROL, &[0]);
    uarto.write(&request).unwrap();
    let mut request = create_modbus(CONTROL_FAROL_ALTO, &[0]);
    uarto.write(&request).unwrap();
    let mut request = create_modbus(CONTROL_FAROL_BAIXO, &[0]);
    uarto.write(&request).unwrap();
    let mut request = create_modbus(CONTROL_WRITE_FAROL_ALTO, &[0]);
    uarto.write(&request).unwrap();

    let valor: f32 = 0.0;
    let bytes = valor.to_le_bytes();
    let mut request = create_modbus(CONTROL_VELOCIDADE_WRITE, &bytes);
    uarto.write(&request).unwrap();
    sleep(Duration::from_millis(50));
    let mut request = create_modbus(CONTROL_RPM_WRITE, &bytes);
    uarto.write(&request).unwrap();
    sleep(Duration::from_millis(50));
    let mut request = create_modbus(CONTROL_RPM_READ, &bytes);
    uarto.write(&request).unwrap();
    sleep(Duration::from_millis(50));
    let mut request = create_modbus(CONTROL_VELOCIDADE_READ, &bytes);
    uarto.write(&request).unwrap();

    sleep(Duration::from_millis(50));
    
}

pub fn velocimetro(speed: f32) {
    // CONTROL_VELOCIDADE e CONTROL_RPM
    let mut uarto = open_uart();
    let bytes = speed.to_le_bytes();
    let mut envia = create_modbus(CONTROL_VELOCIDADE_WRITE, &bytes);
    match uarto.write(&envia) {
        Ok(bytes_escritos) => {

        },
        Err(e) => {
            println!("Erro ao escrever dados: {}", e);
        },
    }

    sleep(Duration::from_millis(50));

    let rpm = speed*8.42;
    let bytes2 = rpm.to_le_bytes();
    let mut envia2 = create_modbus(CONTROL_RPM_WRITE, &bytes2);
    match uarto.write(&envia2) {
        Ok(bytes_escritos2) => {

        },
        Err(e) => {
            println!("Erro ao escrever dados: {}", e);
        },
    }
    sleep(Duration::from_millis(500));

    println!("Velocidade atual: {}\nRPM: {}", speed, rpm);

    drop(uarto);
}
