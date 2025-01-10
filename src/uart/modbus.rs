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

pub const CONTROL_SETA_ESQUERDA: ModbusOperation = ModbusOperation {
    code: 0x00,
    subcode: 0x01,  // seta à esquerda
    qtd: Some(1),
};

pub const CONTROL_SETA_DIREITA: ModbusOperation = ModbusOperation {
    code: 0x00,
    subcode: 0x02,  // seta à direita
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

pub fn read_modbus(operation: ModbusOperation, buffer: &[u8]) -> Result<&[u8], String> {
    if buffer.len() < 5 {
        return Err("Resposta muito curta".to_string());
    }

    let crc_check = checa_crc16(buffer);
    if !crc_check {
        return Err("Erro de CRC".to_string());
    }

    Ok(buffer)
}

#[derive(Clone, Copy)]
pub struct ModbusOperation {
    code: u8,
    subcode: u8,
    qtd: Option<u8>,
}


pub fn temp_motor() {
    let mut uarto = Uart::new(BAUD_RATE, Parity::None, DATA_BITS, STOP_BITS).unwrap();
    uarto.set_read_mode(1, Duration::from_secs(1)).unwrap();

    let mut envia = Vec::with_capacity(9);
    envia.push(0x01);
    envia.push(0x23);
    envia.push(0xAA);
    envia.extend_from_slice(&[5,0,0,7]);

    let crc = crc::hash(&envia);
    envia.extend(&crc.to_le_bytes());

    uarto.write(envia.as_slice()).unwrap(); // Envia mensagem via tx

    sleep(Duration::from_secs(1));

    let mut leitura = vec![0; 255];
    uarto.read(&mut leitura).unwrap(); // Lê buffer 255 caracteres da porta rx
    let float_bytes = &leitura[3..7];
    let float_value = f32::from_le_bytes(float_bytes.try_into().expect("Falha na conversão para f32"));
    println!("Valor do float: {}", float_value);
    drop(uarto);
}

pub fn seta_esquerda() {
    let mut uarto = Uart::new(BAUD_RATE, Parity::None, DATA_BITS, STOP_BITS).unwrap();
    uarto.set_read_mode(1, Duration::from_secs(1)).unwrap();

    let mut envia = Vec::with_capacity(11);
    envia.push(0x01);
    envia.push(0x03);
    envia.push(0x00);
    envia.push(1); // quantidade de dados a ler
    envia.extend_from_slice(&[5,0,0,7]);

    let crc = crc::hash(&envia);
    envia.extend(&crc.to_le_bytes());

    uarto.write(envia.as_slice()).unwrap(); // Envia mensagem via tx

    sleep(Duration::from_secs(1));

    let mut leitura = vec![0; 255]; // retorna 0x00, 0x03 e o byte da seta
    let bytes_lidos = uarto.read(&mut leitura); // Lê buffer 255 caracteres da porta rx

    match bytes_lidos {
        Ok(n) => {
            if n > 0 {
                let byte_da_seta = leitura[2];
                let ultimos_dois_bits = byte_da_seta & 0b00000011;
                println!("Os dois últimos bits do byte da seta: {:02b}", ultimos_dois_bits);
                if ultimos_dois_bits == 0b10 {desliga_seta(0x0C, &mut uarto);} // direita
                else if ultimos_dois_bits == 0b01 {desliga_seta(0x0B, &mut uarto);} // esquerda
            }
        },
        Err(e) => {
            eprintln!("Erro na leitura: {}", e);
        }
    }

    
    drop(uarto);
}

fn desliga_seta(dados: u8, uarto: &mut Uart) {
    // mudar de 10 ou 01 para 00 (botão da seta)
    let mut seta = Vec::with_capacity(11);
    seta.push(0x01);
    seta.push(0x06);
    seta.push(0x00); // endereco
    seta.push(1); // quantidade de dados a escrever
    seta.push(0); 
    seta.extend_from_slice(&[5,0,0,7]);
    let crc = crc::hash(&seta);
    seta.extend(&crc.to_le_bytes());
    uarto.write(seta.as_slice()).unwrap();

    // ler o estado da seta 
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

