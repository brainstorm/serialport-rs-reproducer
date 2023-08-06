use clap::Parser;
use hex_literal::hex;
use std::error::Error;
use std::time::Duration;

// Settings
#[derive(Clone, Debug, Parser)]
struct Args {
    #[arg(long)]
    packet_separation_ms: Option<u64>,
    #[arg(long, default_value_t = false)]
    flush_before_drop: bool,
    #[arg(long, default_value_t = 9600)]
    baud: u32,
    port: String,
}

#[allow(clippy::disallowed_names)]
fn main() -> Result<(), Box<dyn Error>> {
    let foo = hex!("ca fe ca fe ca fe");
    let bar = hex!("ba be ba be ba be");
    let baz = hex!("de ad be ef de ad be ef");

    let args = Args::parse();
    let packet_separation = args.packet_separation_ms.map(Duration::from_millis);
    let mut port = serialport::new(args.port, args.baud).open()?;

    port.write_all(&foo)?;
    if let Some(separation) = packet_separation {
        port.flush()?;
        std::thread::sleep(separation);
    }
    port.write_all(&bar)?;
    if let Some(separation) = packet_separation {
        port.flush()?;
        std::thread::sleep(separation);
    }
    port.write_all(&baz)?;

    if args.flush_before_drop {
        port.flush()?;
    }

    Ok(())
}
