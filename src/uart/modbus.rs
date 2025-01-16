use crate::uart::crc;
use std::thread::sleep;
use std::time::Duration;
use rppal::uart::{Parity, Uart};

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

pub const CONTROL_CRUISE_RESUME: ModbusOperation = ModbusOperation {
    code: 0x01,
    subcode: 0x01,  // RES
    qtd: Some(1),
};

pub const CONTROL_CRUISE_CANCEL: ModbusOperation = ModbusOperation {
    code: 0x01,
    subcode: 0x02,  // CANCEL
    qtd: Some(1),
};

pub const CONTROL_CRUISE_SET_PLUS: ModbusOperation = ModbusOperation {
    code: 0x01,
    subcode: 0x04,  // Set + 
    qtd: Some(1),
};

pub const CONTROL_CRUISE_SET_MINUS: ModbusOperation = ModbusOperation {
    code: 0x01,
    subcode: 0x10,  // Set - 
    qtd: Some(1),
};

pub const FAROL_ALTO: ModbusOperation = ModbusOperation {
    code: 0x02,
    subcode: 0x01,  // Set - 
    qtd: Some(1),
};

pub const FAROL_BAIXO: ModbusOperation = ModbusOperation {
    code: 0x02,
    subcode: 0x02,  // Set - 
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

pub fn read_modbus(uarto : &mut Uart, start: u8, end: u8, tam: usize) -> Vec<u8> {
    let mut leitura = vec![0; tam];
    let response = uarto.read(&mut leitura);

    match response {
        Ok(_) => {
            
        }
        Err(e) => {
            eprintln!("Failed to read from UART: {}", e);
            return Vec::new(); 
        }
    }
    
    leitura[start..end].to_vec()  
}


#[derive(Clone, Copy)]
pub struct ModbusOperation {
    code: u8,
    subcode: u8,
    qtd: Option<u8>,
}


pub fn temp_motor() {
    let mut uarto = open_uart();
    let mut envia = create_modbus(LE_TEMP, &[]);

    uarto.write(envia.as_slice()).unwrap(); 
    sleep(Duration::from_secs(1));
    
    let value = read_modbus(&mut uarto,3,7,7);
    let float_value = f32::from_le_bytes(value.try_into().expect("Falha na conversão para f32"));

    println!("Valor do float: {}", float_value);

    drop(uarto);
}

pub fn seta() {
    let mut uarto = open_uart();
    let mut envia = create_modbus(CONTROL_READ_SETA, &[]);
    uarto.write(envia.as_slice()).unwrap(); 
    sleep(Duration::from_secs(1));

    let value = read_modbus(&mut uarto,0,2,3);

    match value {
        Ok(n) => {
            if n > 0 {
                let byte_da_seta = value[2];
                let ultimos_dois_bits = byte_da_seta & 0b00000011;
                println!("Os dois últimos bits do byte da seta: {:02b}", ultimos_dois_bits);
                if ultimos_dois_bits == 0b10 {desliga_seta(1, &mut uarto);} // direita
                else if ultimos_dois_bits == 0b01 {desliga_seta(2, &mut uarto);} // esquerda
            }
        },
        Err(e) => {
            eprintln!("Erro na leitura: {}", e);
        }
    }

    
    drop(uarto);
}

fn desliga_seta(direction: usize, uarto: &mut Uart) {
    // mudar de 10 ou 01 para 00 (botão da seta)
    let mut seta = create_modbus(CONTROL_WRITE_SETA, &[0]);
    uarto.write(seta.as_slice()).unwrap();

    if direction == 1 { 

    }else{
        let mut envia = create_modbus(CONTROL_READ_SETA_ESQUERDA, &[]);
    }

    let mut envia = Vec::with_capacity(11);
    envia.push(0x01);
    envia.push(0x03);
    envia.push(dados);
    envia.push(1); 
    envia.extend_from_slice(&[5,0,0,7]);
    let crc = crc::hash(&envia);
    envia.extend(&crc.to_le_bytes());
    uarto.write(envia.as_slice()).unwrap(); 
    sleep(Duration::from_secs(1));
    let mut leitura = vec![0; 255]; 
    uarto.read(&mut leitura); 
    println!("Estado atual da seta: {}",leitura[2]);

    // mudar o estado da seta
    let mut registrador = Vec::with_capacity(11);
    registrador.push(0x01);
    registrador.push(0x06);
    registrador.push(dados);
    registrador.push(1); 
    registrador.push(!leitura[2]); // se tiver ligado vai pra 0, se tiver desligado vai pra 1
    registrador.extend_from_slice(&[5,0,0,7]);
    let crc = crc::hash(&registrador);
    registrador.extend(&crc.to_le_bytes());
    uarto.write(registrador.as_slice()).unwrap();
}

