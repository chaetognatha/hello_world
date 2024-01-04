This is a default hello world project written in rust. Future of bioinformatics?

# The basics

First install rust. I am using https://rustup.rs/
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Navigate to the parent directory of your coding projects and create the hello_world* project
```
cargo new hello_world
```
Notice that the project is conveniently set up with a hello world script.

Now we can build:
```
❯ cargo build
   Compiling hello_world v0.1.0 (/home/mattis/CodeZone/hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
```
and run:
```
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello_world`
Hello, world!
```

of course you can also run the executable directly after building:
```
❯ target/debug/hello_world
Hello, world!

```



\* rust uses snake or kebab naming conventions, so hello_world and hello-world are accepted

# Quality of life
Add cargo-edit:
```
cargo install cargo-edit
```

Now I will no longer have to edit the Cargo.toml directly, instead I add a crate like this:
```
cargo add clap
```
and voilá! dependencies are updated!