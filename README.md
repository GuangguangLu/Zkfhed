These codes provide a comprehensive evaluation of zkfhed,  a verifiable and scalable blockchain-enhanced federated learning system.

1. Install Rust & zkVM
curl https://sh.rustup.rs -sSf | sh  
Install Risz Zero zkVM
cargo install cargo-binstall  
cargo binstall cargo-risczero  
cargo risczero install

2. testing zkVM
cargo risczero new my_project --guest-name guest_code_for_zk_proof  

3. git clone https://github.com/GuangguangLu/Zkfhed.git

4. FL_zkVM: These codes evaluate the time and memory costs invested by FL nodes during the local model verification stage for various ML algorithms based on ZKPs.
cd FL_ZKVM/LR  
cargo run

5. Privacy_Delegation_Mechanism: This code aims to evaluate the scalability of Zkfhed, including running time, online time, and communication overhead during the delegation learning process.  
cd Privacy_Delegation_Mechanism/ZKVM+FHE+NN  
cargo run

6. Blockchain_Query: This code evaluates the query efficiency of the transaction indexing mechanism in Zkfhed.  
cd Blockchain_Query  
cargo run  

7. Data_Proof：This code demonstrates the feasibility and evaluates the costs of the training data screening stage.
Install tlsn
git clone https://github.com/tlsnotary/tlsn.git"
cd Data_Proof
cargo run
