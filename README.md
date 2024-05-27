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
 
4. test zkVM  
cargo risczero new my_project  
cd my_project  
cargo run  
Successfully running will display:    
    Finished `dev` profile [optimized + debuginfo] target(s) in 1m 35s  
    Running `target/debug/host`  

5. git clone https://github.com/GuangguangLu/Zkfhed.git  
cd Zkfhed  

4. FL_zkVM: These codes evaluate the time and memory costs invested by FL nodes during the local model verification stage for various ML algorithms based on ZKPs.  
cd FL_ZKVM/LR  
cargo run

6. Privacy_Delegation_Mechanism: This code aims to evaluate the scalability of Zkfhed, including running time, online time, and communication overhead during the delegation learning process.  
cd Privacy_Delegation_Mechanism/ZKVM+FHE+NN  
cargo run

7. Blockchain_Query: This code evaluates the query efficiency of the transaction indexing mechanism in Zkfhed.  
cd Blockchain_Query  
cargo run  

8. Data_Proof：This code demonstrates the feasibility and evaluates the costs of the training data screening stage.  
Install tlsn (https://docs.tlsnotary.org/quick_start/rust.html)     
git clone https://github.com/tlsnotary/tlsn.git"  
cd tlsn/tlsn/examples/twitter  
Modify files： Zkfhed\Data_Proof  (https://github.com/tlsnotary/tlsn/blob/main/tlsn/examples/twitter/README.md)  
cargo run
