use crate::i2c::bmp280::read_bmp280;

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_read_bmp280() {
        let i2c_device = "/dev/i2c-1";
        let device_address = 0x76;

        match read_bmp280(i2c_device, device_address) {
            Ok((temperature, pressure)) => {
                println!("Temperatura: {:.2} ºC", temperature);
                assert!(temperature > -40.0 && temperature < 85.0, "Temperatura fora do intervalo esperado!");
            }
            Err(e) => {
                panic!("Erro ao ler o sensor: {}", e); 
            }
        }
    }
}
