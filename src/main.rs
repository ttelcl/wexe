use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let exe = env::current_exe()?;
    println!("Current executable: {:?}", exe);
    let cfg = exe.with_extension("toml");
    println!("Current cfg:        {:?}", cfg);
    println!("Args:");
    for arg in env::args() {
        println!("+ {}", arg);
    }
    Ok(())
}
