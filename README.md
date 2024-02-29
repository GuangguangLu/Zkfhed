1. Deploy zkvm
Install Rust  
curl https://sh.rustup.rs -sSf | sh  
Install Risz Zero  
cargo install cargo-binstall  
cargo binstall cargo-risczero  
cargo risczero install

2. testing zkvm  
cargo risczero new my_project --guest-name guest_code_for_zk_proof  
git clone https://github.com/GuangguangLu/ZKFed.git

3. FL+ZKVM:Perform zkp on multiple machine learning algorithms  
cd FL_ZKVM/LR  
cargo run

4. Delegation:Perform zkp on encrypted data  
cd Privacy_Delegation_Mechanism/ZKVM+FHE+NN  
cargo run

5. Blockchain Query  
cd Blockchain_Query  
cargo run  
