use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // IP address of the PLC
  let plc_ip = "192.168.2.3";

  // AMS address (Address Management Service) of the PLC
  let plc_ams = [192, 168, 2, 3, 1, 1];

  // Run the "add_route/main.py" Python script and capture its output
  let output = Command::new("python3")
      .arg("add_route/main.py") // Argument for the Python script
      .output()
      .expect("Failed to run Python script"); // Error message

  if output.status.success() {
    // Create an ADS client if the Python script ran successfully
    let client = ads::Client::new(
        (plc_ip, ads::PORT),
        ads::Timeouts::none(),
        ads::Source::Auto,
    )?;

    // Create an object representing the PLC device (using AmsAddr)
    let device = client.device(ads::AmsAddr::new(plc_ams.into(), 851));

    // Check the PLC state (is it running?)
    if device.get_state()?.0 != ads::AdsState::Run {
      return Err("PLC instance is not running".into());
    }

    // Create a handle for a variable in the PLC ("GVL.VARIABLE")
    let handle = ads::Handle::new(device, "GVL.VARIABLE")?;

    // Read a value from the PLC (of type u32)
    let value: u32 = handle.read_value()?;
    println!("GVL.VARIABLE old value is {}", value);

    // Write a value to the PLC
    // let value_to_write: u32 = 1;
    // handle.write_value(&value_to_write)?;

    println!("Python script executed successfully: {:?}", output);
  } else {
    // Print an error message if the Python script failed
    println!("Error: {:?}", output);
  }

  Ok(())
}
