use rpi_embedded::i2c::I2c;
use std::thread::sleep;
use std::time::Duration;
use std::error::Error;

// MAX17320 registers
const REG_CONFIG: u8 = 0x0C;
const REG_VCELL_MIN: u8 = 0x0D;
const REG_VCELL_MAX: u8 = 0x0E;
const REG_NUM_CELLS: u8 = 0x0F;
const REG_STATUS: u8 = 0x19;
const REG_SOC: u8 = 0x1C;

// Configuration values
const CELL_CONFIG_2S: u8 = 0x07;
const VCELL_MIN: u16 = 4200;
const VCELL_MAX: u16 = 4200;
const NUM_CELLS: u8 = 2;

fn main() -> Result<(), Box<dyn Error>> {
    let mut i2c = I2c::new()?;
    i2c.set_slave_address(0x36)?;
    // Configure the MAX17320
    let data = [REG_CONFIG, CELL_CONFIG_2S];
    i2c.write(&data)?;
    let data = [REG_VCELL_MIN, (VCELL_MIN >> 8) as u8, VCELL_MIN as u8];
    i2c.write(&data)?;
    let data = [REG_VCELL_MAX, (VCELL_MAX >> 8) as u8, VCELL_MAX as u8];
    i2c.write(&data)?;
    let data = [REG_NUM_CELLS, NUM_CELLS];
    i2c.write(&data)?;

    // Read the battery status
    loop {
        i2c.write(&[REG_STATUS])?;
        let mut status = [0u8; 1];
        i2c.read(&mut status)?;
        if status[0] & 0x01 == 0x01 {
            // Battery is charging
            println!("Battery is charging");
        } else {
            // Battery is not charging
            println!("Battery is not charging");
        }
        if status[0] & 0x10 == 0x10 {
            // Battery is in low power mode
            println!("Battery is in low power mode");
        } else {
            // Battery is not in low power mode
            println!("Battery is not in low power mode");
        }

        // Read the state of charge (SOC) of the battery
        i2c.write(&[REG_SOC])?;
        let mut soc = [0u8; 1];
        i2c.read(&mut soc)?;
        println!("Battery SOC is {}%", (soc[0] as f32)/2.55);

        // Wait for 5 seconds before reading again
        sleep(Duration::from_secs(5));
    }
}