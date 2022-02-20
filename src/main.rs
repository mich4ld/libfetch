use fetch_os::linux;

fn main() {
    let os = linux::Linux {};
    let memory = os.memory().unwrap();
    let os_name = os.name().unwrap();
    println!("os ~> {}", os_name);
    println!("sh ~> {}", os.shell().unwrap());
    println!("ram ~> {}mb / {}mb", memory.used() / 1024, memory.total / 1024);

}
