# Pimtron

Hey, welcome to Pimtron. This is just my personal portfolio website where I keep my projects and blog posts. I decided to build it using Rust and Leptos, mostly because I wanted to try out full-stack Rust for the web and see how it feels.

If you are curious about the code or want to run it locally to see how it works, here is a quick guide.

## Getting Started

You can set this up the standard way with Cargo, or if you use Nix, I have a flake included to make it easier.

### Standard Setup

First off, you will need to have Rust installed. If you haven't set that up yet, you can grab it from rustup.rs.

Since this runs in the browser, you need to add the WebAssembly target to your Rust toolchain:

```bash
rustup target add wasm32-unknown-unknown
```

You also need a tool called Trunk. It handles the building and serving of the application. You can find the installation instructions on their website:

https://trunkrs.dev/#install

Once you have Rust and Trunk ready, just run this command in the project directory:

```bash
trunk serve
```

That will compile everything and start a local server. You should see it pop up in your browser at localhost:8080.

### Nix Setup

If you are a Nix user, you can skip the manual setup. I have included a `flake.nix` that defines the development environment.

Just run:

```bash
nix develop
```

This will drop you into a shell with Rust, the required WASM target, and Trunk all ready to go. From there, you can just run `trunk serve` as usual.

If you use `direnv`, you can just run `direnv allow` and it will load the environment automatically when you enter the directory.


