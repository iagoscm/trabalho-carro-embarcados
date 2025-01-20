use bmp280::Bmp280Builder;

pub fn read_bmp280(path: &str, address: u16) -> Result<f32, String> {
    let mut dev = Bmp280Builder::new()
        .path(path)
        .address(address)
        .build()
        .map_err(|e| e.to_string())?;

    let temperature = dev.temperature_celsius().map_err(|e| e.to_string())?;

    Ok(temperature)
}

