use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {

  // Run the "add_route/main.py" Python script and capture its output
  let output = Command::new("python3")
      .arg("add_route/main.py") // Argument for the Python script
      .output()
      .expect("Failed to run Python script"); // Error message

  if output.status.success() {
    println!("Python script executed successfully: {:?}", output);
  } else {
    // Print an error message if the Python script failed
    println!("Error: {:?}", output);
  }

  Ok(())
}
