use std::thread::sleep;
use std::time::Duration;

use rppal::uart::Queue;
use rppal::uart::{Parity, Uart};

use crate::uart::crc;
use crate::gpio::gpio;
use crate::car::control::CarControl;

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


pub fn temp_motor() {
    let mut uarto = open_uart();
    let envia = create_modbus(LE_TEMP, &[]);

    uarto.write(envia.as_slice()).unwrap(); 
    sleep(Duration::from_millis(50));
    
    let mut leitura = vec![0; 7];
    let response = uarto.read(&mut leitura).unwrap();
    let float_value = f32::from_le_bytes(leitura[3..7].try_into().expect("Falha na conversão para f32"));

    println!("Valor temp: {}", float_value);

    drop(uarto);
}

pub fn seta(carro: &CarControl) {
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
    if ultimos_dois_bits == 0b10 {muda_seta(1, &mut car_state.seta_direita);} // direita
    else if ultimos_dois_bits == 0b01 {muda_seta(2, &mut car_state.seta_esquerda);} // esquerda
        
    
    drop(uarto);
}

fn muda_seta(direction: usize, estado: &mut bool) {
    let mut uarto = open_uart();
    let mut seta = create_modbus(CONTROL_WRITE_SETA, &[0]);
    uarto.write(&seta).unwrap();
    println!("Direcao: {}",direction);

    if direction == 1 {
        println!("SETA DIREITA");
        //gpio::pisca_seta_direita(); 

        let mut request = create_modbus(CONTROL_WRITE_SETA_DIREITA, &[if *estado { 0 } else { 1 }]);
        *estado = !*estado;
        let mut response = [0; 5];
        uarto.write(&request);
        sleep(Duration::from_millis(50));
        uarto.read(&mut response);

    } else {
        println!("SETA ESQUERDA");
        //gpio::pisca_seta_esquerda(); 

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
        println!("FAROL BAIXO | Estado: {}",*estado);
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
        println!("FAROL ALTO");
        if *estado == true { gpio::farol_alto_desliga(); println!("Liga farol baixo"); }
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

    let mut leitura = vec![0; 4]; 
    uarto.read(&mut leitura);

    let byte_do_cruise = leitura[2];
    match byte_do_cruise {
        0b00000001 => println!("Comando: RES"),
        0b00000010 => println!("Comando: CANCEL"),
        0b00000100 => println!("Comando: SET +"),
        0b00001000 => println!("Comando: SET -"),
        _ => println!("Comando desconhecido: {:08b}",byte_do_cruise),
    }

    let mut farol = create_modbus(CONTROL_CRUISE_WRITE, &[0]);
    uarto.write(&farol).unwrap();

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
}