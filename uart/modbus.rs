use std::str;
use std::thread::sleep;
use std::time::Duration;
use crate::uart::crc;
use std::slice;

use rppal::uart::{Parity, Uart};

const BAUD_RATE: u32 = 115200;
const DATA_BITS: u8 = 8;
const STOP_BITS: u8 = 1;

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
    let bytes_lidos = uarto.read(&mut leitura).unwrap(); // Lê buffer 255 caracteres da porta rx
    let float_bytes = &leitura[3..7];
    let float_value = f32::from_le_bytes(float_bytes.try_into().expect("Falha na conversão para f32"));
    println!("Valor do float: {}", float_value);
    //let mut leitura = vec![0; 255]; // Define 255 caracteres de leitura
    //println!("Leu {} bytes ", bytes_lidos);

    // Converte o buffer para string
    //let to_str_buffer = str::from_utf8(&leitura[0..bytes_lidos]).expect("Não foi possível converter buffer para string").trim_matches(char::from(0));
    //println!("Bytes da leitura: {}", to_str_buffer);

    // Fecha a porta serial
    drop(uarto);
}