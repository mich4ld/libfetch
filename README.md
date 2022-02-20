<h2 align="center">
  <img alt="logo" src="https://user-images.githubusercontent.com/43048524/154862542-47f070af-6e53-4cbf-aeb6-44cfbc1f3305.png" > </br>
  libfetch
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
- no external programs calling;
- multiplatform;
