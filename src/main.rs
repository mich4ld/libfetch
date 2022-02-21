use libfetch::{Info, platform::Platform};

fn main() {
    let os = Info::new();
    
    let username = os.user();
    let hostname = os.hostname();
    let os_name = os.name();
    let kernel = os.kernel();
    let uptime = os.uptime();
    let memory = os.memory().unwrap_or_default();
    let unknown = String::from("unknown");

    println!("{}@{}", username.unwrap_or(unknown.clone()), hostname.unwrap_or(unknown.clone()));
    println!("os ~> {}", os_name.unwrap_or(unknown.clone()));
    println!("kernel ~> {}", kernel.unwrap_or(unknown.clone()));
    println!("uptime ~> {}", uptime.unwrap_or(0));
    println!("ram ~> {} mb / {} mb", memory.used() / 1024, memory.total / 1024);
}
