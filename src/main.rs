use sigmaos_launcher::{Config, LauncherComponent, Component};
fn main() {
    env_logger::init();
    let mut c = LauncherComponent::new(Config::default());
    if let Err(e) = c.initialize() { eprintln!("Failed: {}", e); std::process::exit(1); }
    println!("SigmaOS launcher running");
    let _ = c.shutdown();
}
