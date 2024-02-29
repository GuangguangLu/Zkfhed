1. 部署zkvm环境
安装Rust
curl https://sh.rustup.rs -sSf | sh
安装Risz Zero
cargo install cargo-binstall
cargo binstall cargo-risczero
cargo risczero install
2. 测试环境
cargo risczero new my_project --guest-name guest_code_for_zk_proof
git clone 当前文件
4. FL+ZKVM:Perform zkp on multiple machine learning algorithms
cd FL_ZKVM/LR
cargo run
5. Delegation:Perform zkp on encrypted data
cd Privacy_Delegation_Mechanism/ZKVM+FHE+NN
cargo run
6. 区块链实验
cd Blockchain_Query
cargo run
