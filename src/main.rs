use libfetch::{Info, platform::Platform};

fn main() {
    let os = Info {};
    
    let hostname = os.hostname();
    let os_name = os.name();
    let kernel = os.kernel();
    let memory = os.memory().unwrap_or_default();
    
    println!("user@{}", hostname.unwrap_or(String::from("unknown")));
    println!("os ~> {}", os_name.unwrap_or(String::from("unknown")));
    println!("kernel ~> {}", kernel.unwrap_or(String::from("unknown")));
    println!("ram ~> {} mb / {} mb", memory.used() / 1024, memory.total / 1024);
}
