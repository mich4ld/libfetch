use fetch_os::linux;

fn main() {
    let os = linux::Linux {};
    os.memory();
    let os = os.name().unwrap();
    println!("{}", os);
}
