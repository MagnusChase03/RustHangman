<a name="readme-top"></a>

![issues-shield]
![forks-shield]
![stars-shield]
![license-shield]

<h3 align="center">Rust Hangman</h3>

  <p align="center">
    A simple game written in rust.
    <br />
    ·
    <a href="https://github.com/MagnusChase03/RustHangman/issues">Report Bug</a>
    ·
    <a href="https://github.com/MagnusChase03/RustHangman/issues">Request Feature</a>
  </p>
</div>

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#license">License</a></li>
  </ol>
</details>

## About The Project

Because rust is becoming more and more popular, I decided to get at least the basic fundementals of how it works. After writing my linked list, I decided I needed to learn how to grab user input and mess around with file io. This project also tought me more about how Strings and vectors work within the rust language, and this project further improved my understanding of ownership and lifetimes.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With

* ![rust-shield]
* ![tauri-shield]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Getting Started

### Prerequisites

To build this project yourself, you are going to need rust and tauri

To install rust 

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then install tauri with

```
cargo install tauri
```

### Installation

If you want to build the app yourself

1) Clone the repo
2) Run `cargo tauri build`
3) Find the executable in `src-tauri/target/release/bundle`

If you just want to run the app

1) Go to <a href="https://github.com/MagnusChase03/RustHangman/releases">Releases</a>
2) Download and run the msi
3) Run the app executable

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Usage

Use cargo to run the application

```
cargo tauri dev
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->
## License

Distributed under the GPL License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

[issues-shield]: https://img.shields.io/github/issues/MagnusChase03/RustHangman?style=for-the-badge
[forks-shield]: https://img.shields.io/github/forks/MagnusChase03/RustHangman?style=for-the-badge
[stars-shield]: https://img.shields.io/github/stars/MagnusChase03/RustHangman?style=for-the-badge
[license-shield]: https://img.shields.io/github/license/magnuschase03/RustHangman?style=for-the-badge
[rust-shield]: https://img.shields.io/badge/Rust-20232A?style=for-the-badge&logo=rust
[tauri-shield]: https://img.shields.io/badge/Tauri-20232A?style=for-the-badge&logo=tauri
