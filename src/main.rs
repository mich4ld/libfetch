use fetch_os::linux;

fn main() {
    let os = linux::Linux {};
    let os = os.name().unwrap();
    println!("{}", os);
}
