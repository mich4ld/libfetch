use fetch_os::linux;

fn main() {
    let os = linux::Linux {};
    let memory = os.memory().unwrap();
    let os_name = os.name().unwrap();
    println!("{}@{}", os.user().unwrap(), os.hostname().unwrap());
    println!("os ~> {}", os_name);
    println!("kernel ~> {}", os.kernel().unwrap());
    println!("sh ~> {}", os.shell().unwrap());
    println!("ram ~> {}mb / {}mb", memory.used() / 1024, memory.total / 1024);
    println!("wm ~> {}", os.desktop().unwrap());
    println!("uptime ~> {}h", os.uptime().unwrap() / 60 / 60);
}
