use std::time::Duration;
use std::thread;
use std::fs::File;
use std::io::Write;
use std::process;

use probe_rs::Session;
use probe_rs::MemoryInterface;

fn dump() -> Result<(), probe_rs::Error> {
    let mut session = Session::auto_attach("nrf52")?;
    let mut core = session.core(0)?;

    println!("Attempting read...");

    // Read user config from 0x0, to 0x210 bytes
    let mut buf = [0u8; 0x210];
    core.read_8(0x10001000, &mut buf)?;

    // Write to file
    let mut file = File::create("airtag.uicr.bin").unwrap();
    file.write_all(&buf).unwrap();

    // Read flash from 0x0, 512kb
    let mut buf = [0u8; 0x80000];
    core.read_8(0x0, &mut buf)?;

    // Write to file
    let mut file = File::create("airtag.flash.bin").unwrap();
    file.write_all(&buf).unwrap();

    Ok(())
}


fn main() {
    let mut port = serialport::new("/dev/ttyACM0", 115_200)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");


    loop {
        for glitch in 0..30 {
            // About ~20 us before drop in core voltage due to flash read starting
            // No need to loop over this value as our trigger is very noisy

            let delay = 19500; 
            println!("Width: {:}", glitch);

            // Turn off target power
            port.write(b"l        ").unwrap();

            // Configure glitch params
            port.write(b"d").unwrap();
            port.write(format!("{:08}", delay).as_bytes()).unwrap();

            port.write(b"w").unwrap();
            port.write(format!("{:08}", glitch).as_bytes()).unwrap();

            // Peform glitch
            thread::sleep(Duration::from_millis(50));
            port.write(b"g        ").unwrap();

            // Try attaching to chip.
            match dump() {
                Ok(_) => {
                    println!("Dumped succesfully");
                    process::exit(0);
                },
                Err(e) => {
                    println!("Error dumping: {:?}", e);
                }
            }

        }
    }
}
