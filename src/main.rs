use fetch_os::linux;

fn main() {
    let os = linux::Linux {};
    let memory = os.memory().unwrap();
    let os = os.name().unwrap();
    println!("{}", os);
    println!("{}mb / {}mb", memory.used() / 1024, memory.total / 1024);
}
