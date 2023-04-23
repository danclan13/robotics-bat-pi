// Import the crates
use rpi_embedded::i2c::I2c;
use max17320::MAX17320;
use max17320::Register;

// Define a run function that contains the let statement
fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Create an I2c instance with the default settings
    let mut i2c = I2c::new()?;

    // Set the slave address to 0x6C
    i2c.set_slave_address(0x6C)?;

    // Create a Max17320 instance with the I2c instance

    let mut bat = max17320::MAX17320::new(i2c, 5.0).expect("mx");


    // Configure the device for 2 cell 4.2V batteries
    // Set the number of cells to 2
    bat.write_register(NPackCfg, 0x0001)?;
    // Set the charge voltage to 4.2V per cell
    max17320.write_register(MAX17320_VCHG, 0x00D0)?;
    // Set the termination current to 100mA
    max17320.write_register(MAX17320_ICHGTERM, 0x000A)?;

    // Read the charge status register
    let status = max17320.read_register(MAX17320_STATUS)?;

    // Check the bits for charge status
    let charging = status & 0x0001 != 0; // Bit 0
    let done = status & 0x0002 != 0; // Bit 1
    let suspend = status & 0x0004 != 0; // Bit 2

    // Print the charge status
    if charging {
        println!("The batteries are charging.");
    } else if done {
        println!("The batteries are fully charged.");
    } else if suspend {
        println!("The charging is suspended.");
    } else {
        println!("The batteries are not charging.");
    }

    Ok(())
}

fn main() {
    // Call the run function and handle errors
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}