# A Rust/WASM playground hello world repo

### Dev Environment Setup 

* Install Rust.  On Windows/WSL: 

`$>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

* Install `wasm-pack`. 

`$>cargo install wasm-pack`

* (optional) Install nvm/node.js/npm/http-serve

`$>curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.34.0/install.sh | bash                                                                                                      389 `

`$>source ~/.bashrc`

`$>npm install -g http-server`

### Running

Serve from a local directory with

`$>http-server -p 8080`

and go to http://localhost:8080
