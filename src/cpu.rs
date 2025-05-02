use std::error::Error;

pub async fn cpus() -> Result<u32, Box<dyn Error>> {
    let cpus = num_cpus::get();
    Ok(cpus as u32)
}
