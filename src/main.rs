use std::error::Error;
use hex_literal::hex;

// Settings
const PORT: &str = "/dev/cu.usbserial-0001";
const BAUD: u32 = 9600;

fn main () -> Result<(), Box<dyn Error>> {
	let foo= hex!("ca fe ca fe ca fe");
	let bar = hex!("ba be ba be ba be");
	let baz = hex!("de ad be ef de ad be ef");

	let mut port = serialport::new(PORT, BAUD)
		.open()?;

	port.write_all(&foo)?;
	port.write_all(&bar)?;
	port.write_all(&baz)?;

	Ok(())
}
