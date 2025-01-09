
use std::thread::sleep;
use std::time::Duration;
use crate::uart::crc;

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