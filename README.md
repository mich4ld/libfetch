<h2 align="center">
  <img alt="logo" src="https://user-images.githubusercontent.com/43048524/154870178-f8a20dac-7ff6-4332-8724-4c689ac03d4f.png" ></br></br>
  libfetch [work in progress]
</h2>

<p align="center">
  Simple but reliable library for fetch tools written in Rust 🦀.
</p>
  
 
### Overview
It is hard to create cool fetch info tool for Linux, so this library tries simplify that process.
Library is not calling any other programs like `uname -n` or `uptime`. It is trying to fetch OS
info in more reliable way.

### Goals
- avoid unsafe blocks (but not always possible);
- avoid calling external programs;
- multiplatform;
- simple API;

### Alternatives
- <a href="https://github.com/Macchina-CLI/libmacchina">Macchina-CLI/libmacchina</a>
- <a href="https://github.com/GuillaumeGomez/sysinfo">GuillaumeGomez/sysinfo</a>
- <a href="https://github.com/FillZpp/sys-info-rs">FillZpp/sys-info-rs</a>
