extern crate probe_rs;
use probe_rs::Probe;
use probe_rs::DebugProbeError;

fn main() -> Result<(), DebugProbeError> {
    println!("Hello, probe-rs!");
    // Get a list of all available debug probes.
    let probes = Probe::list_all();
    println!("{:?}", probes);
    
    // Use the first probe found.
    let probe = probes[0].open()?;
    println!("{:?}", probe);

    // Attach to a chip.
    let mut session = probe.attach("stm32h7").unwrap();
    println!("{:?}", session);
    
    // Select a core.
    let mut core = session.core(0).unwrap();

    // Halt the attached core.
    core.halt(std::time::Duration::from_secs(3)).unwrap();
    
    std::thread::sleep(std::time::Duration::from_secs(3));

    // Run the attached core
    core.run().unwrap();

    Ok(())
}
