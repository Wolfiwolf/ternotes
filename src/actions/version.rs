pub fn _version() -> Result<(), Box<dyn std::error::Error>> {
    println!("\nVersion: v{}\n", env!("CARGO_PKG_VERSION"));
    Ok(())
}
