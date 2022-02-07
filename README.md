# bevy_clap

A bevy plugin to parse command line arguments with clap.

## Usage

```rust
#[derive(clap::Parser)]
#[clap(name = "demo")]
struct Context {
   /// More verbose output
   #[clap(long)]
   verbose: bool,
   /// An optional name
   #[clap(short, long)]
   name: Option<String>,
}

fn main() {
    App::new()
        // ...
        .add_plugin(ClapPlugin::<Context>::default())
        // ...
        .run();
}
```

The `Context` value will be added as a clap resource on startup.
