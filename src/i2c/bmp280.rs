use bmp280::Bmp280Builder;

pub fn read_bmp280(path: &str, address: u16) -> Result<(f32, f32), String> {
    let mut dev = Bmp280Builder::new()
        .path(path)
        .address(address)
        .build()
        .map_err(|e| e.to_string())?;

    let temperature = dev.temperature_celsius().map_err(|e| e.to_string())?;
    let pressure = dev.pressure_kpa().map_err(|e| e.to_string())?;
//    todo!("Converter kpa para hpa");

    Ok((temperature, pressure))
}

/*
Codigo de exemplo para puxar a temperatura:
  match i2c::bmp280::read_bmp280("/dev/i2c-1", 0x76) {
      Ok((temperature, pressure)) => {
      // Exibe os valores de temperatura e pressão
          println!("Temperatura: {:.2} ºC", temperature);
          println!("Pressão: {:.2} kPa", pressure);
        }
      Err(e) => {
          println!("Erro ao ler o sensor: {}", e);
        }
  }

*/
