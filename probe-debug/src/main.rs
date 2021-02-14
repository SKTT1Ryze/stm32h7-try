extern crate probe_rs;
use probe_rs::Probe;
use probe_rs::DebugProbeError;
use std::time::Duration;
use std::thread;

fn main() -> Result<(), DebugProbeError> {
    println!("Hello, probe-rs!");
    // Get a list of all available debug probes.
    let probes = Probe::list_all();
    println!("{:?}", probes);
    
    // Use the first probe found.
    let probe = probes[0].open()?;

    // probe.select_protocol(WireProtocol::Swd).unwrap();

    println!("{:?}", probe);
    
    // Attach to a chip.
    let mut session = probe.attach("stm32h7").unwrap();
    println!("{:?}", session);

    let mut core = session.core(0).unwrap();

    // Reset and halt the attached core
    core.reset_and_halt(Duration::from_millis(3)).unwrap();

    thread::sleep(Duration::from_secs(2));
    
    // Run the attached core
    core.run().unwrap();
    Ok(())
}
