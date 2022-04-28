<h2 align="center">
  <img alt="logo" src="https://user-images.githubusercontent.com/43048524/154870178-f8a20dac-7ff6-4332-8724-4c689ac03d4f.png" ></br></br>
  libfetch
</h2>

<p align="center">
  Simple but reliable library for fetch tools written in Rust 🦀.
</p>

#### Note: it is experimental and early version of crate
 
### Overview
It's hard to create a cool fetch tool for Linux, so this library tries to simplify the process. The library does not call any other programs, such as "uname -n" or "uptime". It tries to fetch the operating system information in a more reliable way.

### Goals
- avoid unsafe blocks (but not always possible);
- avoid calling external programs;
- multiplatform;
- simple API;

### To-Do
- write tests
- add support for Windows and handle unknown OS

### Alternatives
- <a href="https://github.com/Macchina-CLI/libmacchina">Macchina-CLI/libmacchina</a>
- <a href="https://github.com/GuillaumeGomez/sysinfo">GuillaumeGomez/sysinfo</a>
- <a href="https://github.com/FillZpp/sys-info-rs">FillZpp/sys-info-rs</a>
- <a href="https://github.com/unrelentingtech/systemstat">unrelentingtech/systemstat</a>
