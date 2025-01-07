mod i2c;
mod uart;

use uart::modbus::temp_motor;

fn main() {
  temp_motor();
}
