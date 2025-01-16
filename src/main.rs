mod i2c;
mod uart;
mod gpio;

//use std::thread;
use uart::modbus::seta;
use uart::modbus::temp_motor;
//use std::time::Duration;

fn main() {
  // a cada 50ms pedir temperatura do motor, e ler comando das setas
    loop {
        // let temp_motor_data = create_modbus(LE_TEMP, &[]);
        // let seta_esquerda_data = create_modbus(CONTROL_SETA_ESQUERDA, &[1]);
        // let seta_direita_data = create_modbus(CONTROL_SETA_DIREITA, &[1]);

        // let response_temp = read_modbus(LE_TEMP, &temp_motor_data);
        // let response_seta_esquerda = read_modbus(CONTROL_SETA_ESQUERDA, &seta_esquerda_data);
        // let response_seta_direita = read_modbus(CONTROL_SETA_DIREITA, &seta_direita_data);

        // match response_temp {
        //     Ok(response) => {
        //         println!("Temperatura do motor: {:?}", response);
        //     }
        //     Err(e) => {
        //         println!("Erro ao ler temperatura: {}", e);
        //     }
        // }

        // match response_seta_esquerda {
        //     Ok(response) => {
        //         println!("Comando de seta à esquerda recebido: {:?}", response);
        //     }
        //     Err(e) => {
        //         println!("Erro ao ler comando de seta à esquerda: {}", e);
        //     }
        // }

        // match response_seta_direita {
        //     Ok(response) => {
        //         println!("Comando de seta à direita recebido: {:?}", response);
        //     }
        //     Err(e) => {
        //         println!("Erro ao ler comando de seta à direita: {}", e);
        //     }
        // }
        temp_motor();
        seta();
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
