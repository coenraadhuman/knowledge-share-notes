_[Introduction](./introduction.md) << Basics >> [Types and Variables](./types-and-variables.md)_

# Installing

They have a nice _Getting Started_ page available at [rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started) that details the install process with an utility called Rustup.

If you just want to try it without installing you try it online at [play.rust-lang.org](https://play.rust-lang.org/).

__Rust has its own books and material available online at [rust-lang.org/learn](https://www.rust-lang.org/learn).__

# Build Tool and Package Manager

__Cargo__ comes with Rustup :smile:.

It includes:
- build your project with `cargo build`.
- run your project with `cargo run`.
- test your project with `cargo test`.
- build documentation for your project with `cargo doc`.
- publish a library to crates.io with `cargo publish`.

Cargo has a official book available for free online, [doc.rust-lang.org/cargo/index.html](https://doc.rust-lang.org/cargo/index.html), detailing its use.

# IDEs

Rust supports multiple IDEs, the most noteworthy is `IntelliJ` and `Visual Studio` that comes with support natively, no external third-party plugins.

Some other editors with some support _(setup process detailed in [getting started page](https://www.rust-lang.org/learn/get-started))_:

- VS Code
- Sublime Text 3
- Atom
- Eclipse
- VIM 
- Emacs
- Geany

# Generating a New Project

Made simple using cargo: `cargo new hello-rust` _(terminal based project with hello-rust being the project name)_.

The project structure includes:
```
hello-rust
|- Cargo.toml
|- src
  |- main.rs
```

- `Cargo.toml` is the manifest file for Rust. It’s where you keep metadata for your project, as well as dependencies.
- `src/main.rs` is where we’ll write our application code.