These codes provide a comprehensive evaluation of zkfhed,  a verifiable and scalable blockchain-enhanced federated learning system.

1. Install Rust  
(https://www.rust-lang.org/zh-CN/tools/install)  
sudo apt update  
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  
export PATH="$HOME/.cargo/bin:$PATH"  
rustc --version  

2. Install Risz Zero zkVM  
sudo apt install curl build-essential libssl-dev pkgconf  
cargo install cargo-binstall  
cargo binstall cargo-risczero    (Note: yes must be entered here)    
cargo risczero install  
 
3. test zkVM  
cargo risczero new my_project  
cd my_project  
cargo run  
Successfully running will display:    
    Finished `dev` profile [optimized + debuginfo] target(s) in 1m 35s  
    Running `target/debug/host`  

4. git clone https://github.com/GuangguangLu/Zkfhed.git  (root directory)  

5. Blockchain_Query: This code evaluates the query efficiency of the transaction indexing mechanism in Zkfhed.  
cd Zkfhed/Blockchain_Query  
cargo run

6. Data_Proof：This code demonstrates the feasibility and evaluates the costs of the training data screening stage.  
Install tlsn (https://docs.tlsnotary.org/quick_start/rust.html)     
git clone https://github.com/tlsnotary/tlsn.git  (root directory)  
--------TEST--------  
cd tlsn/tlsn/examples/simple  
cargo run --release --example simple_prover  
IF ERROR：  
Delete 265 lines: /root/tlsn/components/tls/tls-client/src/lib.rs  (unused_qualifications)  
again：cargo run --release --example simple_prover  
--------TEST--------  
(You must set sensitive information such as Tokens and Authorization based on your account.)  
Taking Discord data as an example:(https://docs.tlsnotary.org/quick_start/rust.html#rust-simple)  
cd tlsn/notary-server  
cargo run --release  
(Start a new command)  
cd tlsn/tlsn/examples/discord  
Modify the Modify the env.example file and name it env. (The env file in Zkfhed is no longer valid)  
RUN: RUST_LOG=debug,yamux=info cargo run --release --example discord_dm  
Other Twitter example:(Detailed steps:https://github.com/tlsnotary/tlsn/blob/main/tlsn/examples/twitter/README.md)  

7. FL_zkVM: These codes evaluate the time and memory costs invested by FL nodes during the local model verification stage for various ML algorithms based on ZKPs.
(Due to the rapid update of zkVM versions, you must pay attention to the new directory structure)  
cd my_project
(Update the corresponding files using the Cargo.toml and main.rs in FL_zkVM/LR/host or method. Please be careful not to break the original code structure.)  
cargo run

8. Privacy_Delegation_Mechanism: This code aims to evaluate the scalability of Zkfhed, including running time, online time, and communication overhead during the delegation learning process.  
cd my_project
(Same as above:Update the corresponding files)  
cargo run


