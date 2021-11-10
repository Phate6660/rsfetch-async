use std::error::Error as Error;
use nixinfo_async::{cpu, dist};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cpu = cpu().await?;
    let distro = dist("/etc/os-release").await?;
    let mut info = std::collections::HashMap::new();
    info.insert("cpu", cpu);
    info.insert("distro", distro);
    std::thread::spawn(|| {
        for (key, value) in info {
            println!("{: <6} = {}", key, value);
        }
    });
    Ok(())
}
