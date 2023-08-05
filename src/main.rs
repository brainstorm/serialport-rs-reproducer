use std::{error::Error, thread::sleep, time::Duration};
use hex_literal::hex;

// Settings
const PORT: &str = "/dev/cu.usbserial-00000000";
const BAUD: u32 = 9600;
const CMD_WAIT: usize = 10;

fn wait(milis: usize) {
	sleep(Duration::from_millis(milis as u64));
}

fn main () -> Result<(), Box<dyn Error>> {
	let foo= hex!("ca fe ca fe ca fe");
	let bar = hex!("ba be ba be ba be");
	let baz = hex!("de ad be ef de ad be ef");

	let mut port = serialport::new(PORT, BAUD)
		.flow_control(serialport::FlowControl::Software)
		.open()?;

	wait(CMD_WAIT);
	port.write(&foo)?;
	wait(CMD_WAIT);
	port.write(&bar)?;
	wait(CMD_WAIT);
	port.write(&baz)?;
	// Uncomment this and the last packet gets sent successfully
	//
	//wait(CMD_WAIT);
	//
	// None of those work to substitute the above wait(CMD_WAIT):
	// port.flush()?;
	// drop(port);

	Ok(())
}
