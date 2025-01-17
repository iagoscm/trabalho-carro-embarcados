/*
use crate::i2c::bmp280::read_bmp280;

#[test]
fn measure(){
    // Arrange
    let mut dev = Bmp280Builder::new()
        .path(path)
        .address(address)
        .build()
        .map_err(|e| e.to_string());

    // Act
    let temperature = dev.temperature_celsius().map_err(|e| e.to_string());
    let pressure = dev.pressure_kpa().map_err(|e| e.to_string());

    let mut dev = read_bmp280("/dev/i2c-1", 0x76);
    let temperature = dev.temperature_celsius().map_err(|e| e.to_string())?;
    let pressure = dev.pressure_kpa().map_err(|e| e.to_string())?;

    // Assert
    assert!(dev.temperature > 0.0 && temperature < 50.0);
    assert!(pressure > 60.0 && pressure < 100.0);
}
*/
use crate::i2c::bmp280::read_bmp280;

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_read_bmp280() {
        // Caminho e endereço do dispositivo
        let i2c_device = "/dev/i2c-1";
        let device_address = 0x76;

        match read_bmp280(i2c_device, device_address) {
            Ok((temperature, pressure)) => {
                // Verifica se os valores retornados estão dentro de um intervalo esperado
                println!("Temperatura: {:.2} ºC", temperature);
                println!("Pressão: {:.2} kPa", pressure);

                // Supondo que os valores esperados sejam conhecidos
                assert!(temperature > -40.0 && temperature < 85.0, "Temperatura fora do intervalo esperado!");
                assert!(pressure > 30.0 && pressure < 110.0, "Pressão fora do intervalo esperado!");
            }
            Err(e) => {
                panic!("Erro ao ler o sensor: {}", e); // Falha no teste se der erro
            }
        }
    }
}
