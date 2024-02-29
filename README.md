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
4. FL+ZKVM 对多种机器学习算法进行零知识证明
cd FL_zkvm
