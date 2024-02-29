//curl https://sh.rustup.rs -sSf | sh
use sha2::{Sha256, Digest};
use rand::Rng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

use rand::rngs::StdRng;

use std::fmt;
use std::collections::HashMap;
use std::sync::Arc;    // trait 有七个函数
use eth_trie::MemoryDB;
use eth_trie::{EthTrie, Trie, TrieError};
use std::time::Instant;

use tiny_keccak::{Hasher, Keccak};

// 使用静态字符串作为指向最后一个区块头的全局key
static GLOBAL_BLOCK_KEY: [u8; 32] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31, 32,
];


// 定义区块头结构体  //BlockHeader_MAP:的key是[u8; 256]，value是[u8; 256]。  但是可以转为16进制。
// hash就是16进制，等于10进制的[u8; 32]
#[derive(Clone, Debug)]
struct BlockHeader {
    bloom: [u8; 256],
    headerhash: [u8; 32],
}





#[derive(Clone, Debug)]  //应该有两种类型的交易，并且每个交易中只有一个指针
struct Transaction {  //RN相同，CID不同，用FHP建立索引
    From: [u8; 20],
    To: [u8; 20],
    RN: [u8; 1],
    CID: [u8; 32],  //存储IPFS数据
    FHP: [u8; 32],
}

struct TransactionTWO {  //元数据
    From: [u8; 20],
    To: [u8; 20],
    Metadata: [u8; 32],
    SHP: [u8; 32],
}


fn generate_random_bytes(rng: &mut StdRng, length: usize) -> Vec<u8> {
    (0..length).map(|_| rng.gen()).collect()
}

// 计算 Transaction 的哈希值，不包括FHP本身
fn calculate_transaction_hash(transaction: &Transaction) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(&transaction.From);
    hasher.update(&transaction.To);
    hasher.update(&transaction.RN);
    hasher.update(&transaction.CID);
    let hash = hasher.finalize();
    let mut result = [0u8; 32];
    result.copy_from_slice(&hash[..]);
    result
}

fn concatenate_transaction_data(transaction: &Transaction) -> Vec<u8> {  //拼接数据，补考看FHP
    let mut concatenated_data = Vec::new();
    
    // 拼接 From 字段
    concatenated_data.extend_from_slice(&transaction.From);
    concatenated_data.extend_from_slice(&transaction.To);
    concatenated_data.extend_from_slice(&transaction.RN);
    concatenated_data.extend_from_slice(&transaction.CID);
    concatenated_data.extend_from_slice(&transaction.FHP);
    // 输出拼接后的数据
    //println!("Concatenated Transaction Data: {:?}", concatenated_data);

    concatenated_data
}


fn set_bloom_bits(bloom: &mut [u8; 256], hash: &[u8; 32]) {
    for i in 0..3 {   // 循环遍历，add计算出的哈希值的前三个字节:
        let bit_index = ((hash[i] as usize) + ((hash[i + 1] as usize) << 8)) % (256 * 8);
        let byte_index = bit_index / 8;
        let bit = 1 << (bit_index % 8);   //选中一位
        bloom[byte_index] |= bit;  //置为1
    }
}


//利用混合后的交易数组，输出多颗MPT树。 返回一个根hash数组。  这个数组的每个元素，就是区块头中的一个hash
fn create_mpt_trees(count: usize, merged_array: Vec<&Transaction>) -> (Vec<[u8; 32]>, Vec<Arc<MemoryDB>>, Vec<BlockHeader>) {
    let mut root_hashes = Vec::new();  //保存根hash，应该是区块头中的hash
    let mut memdbs = Vec::new(); // 保存内存中的每个MPT实例，用于查找BF后查找MPT
    let mut block_headers = Vec::new();  //保存多个区块头
    
    for chunk in merged_array.chunks(count) {
        let memdb = Arc::new(MemoryDB::new(true));
        let mut trie = EthTrie::new(memdb.clone());
        let mut mul_address: Vec<String> = Vec::new();  //用于记录多个交易的地址，计算BF
        let mut concatenated_values = Vec::new(); // 用于记录所有交易的数据
        
        for element in chunk {
            let value = concatenate_transaction_data(element);   //包括FHP//获取到from，也就是地址，然后建立BF
            let address = element.From;
            //println!("address: {:?}", hex::encode(address.clone()));   //16进制
            mul_address.push(hex::encode(address.clone()));  //得到一颗MPT树中的所有地址，保存在mul_address中
            
            concatenated_values.extend_from_slice(&value); //拼接所有交易的数据，包括FHP
            //其实包括不包括无所谓，因为区块头中的hash作用是建立MAP，随机的key也行，只要在MAP中用2→1
            let mut hasher = Sha256::new();
            hasher.update(&value);
            let result = hasher.finalize();

            let hex_hash = format!("{:x}", result);
            //println!("hash: {:?}", hex::encode(hex_hash.clone()));
            trie.insert(hex_hash.as_bytes(), &value).unwrap();  //建立MPT时，key和value也包括FHP
        }
        
        let blockheader_hash = Sha256::digest(&concatenated_values);  //拼接后数据的hash
        //println!("设置BF前的mul_address{:?} ",mul_address );
        let newaddress = hex_strings_to_bytes(mul_address.clone());
        //println!("newaddress{:?} ",newaddress );  //16进制，转化为vec[u8]后，BF判断成功。
        let mut bloom = [0u8; 256];
        for address in newaddress.iter() {
            let mut hasher = Keccak::v256();
            let mut output = [0u8; 32];  //32字节 == 256位
            
            hasher.update(address);
            hasher.finalize(&mut output);
            set_bloom_bits(&mut bloom, &output);  //调用，set_bloom_bits，将区块头中的BF置为1。
        }

        let root_hash = trie.root_hash().unwrap();
        root_hashes.push(root_hash.into());
        memdbs.push(memdb); // 保存 MemoryDB 的引用
        
        let block_header = BlockHeader {
            bloom,
            headerhash: blockheader_hash.try_into().expect("Hash conversion error"),//保存了自己所有数据的hash。
        };
        block_headers.push(block_header);
    }

    (root_hashes, memdbs,block_headers) // 返回 root_hashes 和对应的 MemoryDB 实例
}

fn check_address_in_bloom(bloom: &[u8; 256], address: &[u8; 20]) -> bool {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(address);
    hasher.finalize(&mut output);

    for i in 0..3 {
        let bit_index = ((output[i] as usize) + ((output[i + 1] as usize) << 8)) % (256 * 8);
        let byte_index = bit_index / 8;
        let bit = 1 << (bit_index % 8);
        if bloom[byte_index] & bit == 0 {
            // 如果任何一个位没有被设置，说明地址不可能存在于布隆过滤器中
            return false;
        }
    }
    // 如果所有相关位都被设置，那么地址可能存在于布隆过滤器中
    true
}

fn hex_strings_to_bytes(hex_strings: Vec<String>) -> Vec<Vec<u8>> {
    hex_strings.iter().filter_map(|hex_str| {
        hex::decode(hex_str).ok()
    }).collect()
}


fn main() {
    let seed = [0; 32]; // 这是一个固定的种子
    let mut rng = StdRng::from_seed(seed);
    
    //生成一批建立索引的交易，并且存储在transactions中。
    let mut transactions = Vec::new();
    let mut total_map = HashMap::new();  //交易的MAP
    let mut prev_hash = [0u8; 32]; // 初始哈希值，用于第一个交易
    
    for _ in 0..1000 { // 变量1:链上总的交易数量
        let transaction = Transaction {
            From: generate_random_bytes(&mut rng,20).try_into().unwrap(),
            To: generate_random_bytes(&mut rng,20).try_into().unwrap(),
            RN: generate_random_bytes(&mut rng,1).try_into().unwrap(),
            CID: generate_random_bytes(&mut rng,32).try_into().unwrap(),
            FHP: prev_hash, // 设置为前一个交易的哈希值
        };

        prev_hash = calculate_transaction_hash(&transaction); // 计算当前交易的哈希值，不包括FHP，用于下一个交易
        
        //println!("第_笔交易的hash值为{}", hex::encode(prev_hash));
        //println!("第_笔交易的FHP值为{}", hex::encode(transaction.FHP));
        total_map.insert(prev_hash, transaction.clone());  //把当前交易的hash和data包括FHP，插入MAP
        transactions.push(transaction);
    }

    //建利用建立好索引的交易数组transactions，建立MPT树
    
    let count = 10; // 变量2：每个区块中，即每颗MPT树种包括多少交易。  一个区块中的交易就是一颗树的交易。
    
    let (root_hashes,memdbs,block_headers) = create_mpt_trees(count, transactions.iter().collect());  //已经得到4个MPT树的根hash
    let mut blockheader_map = HashMap::new();  //第二个MAP：区块2的hash指向区块1的BF
    for i in 1..block_headers.len() {
        let key = block_headers[i].headerhash;
        let value = block_headers[i - 1].clone();
        blockheader_map.insert(key, value);
    }
    
    /*********************************   区块头索引           *************************************
    遍历向量中的每个元素并插入map中。其中：key为第二个结构体的headerhash，value为第一个结构体的BF。  key为第三个结构体的headerhash，value为第二个结构体的BF。  key为第四个结构体的headerhash，value为第三个结构体的BF。
    我需要一个全局头指针，保存最后一颗树种所有数据的hash，也就是指向第一个区块头的hash。
    我有最新的交易索引指针，就是：let mut current_hash = prev_hash; 
    ****************************************************************************/
    if let Some(last_block_header) = block_headers.last() {  //找到最后一个区块，插入MAP
        //let global_block_key = GLOBAL_BLOCK_KEY.to_string(); // 将静态str转换为String以匹配HashMap的key类型
        blockheader_map.insert(GLOBAL_BLOCK_KEY, last_block_header.clone());
    }
    //区块头MAP，索引MAP，都建立好了，比较查询时间。
    
    
    //遍历所有区块头
    let mut header_hash = GLOBAL_BLOCK_KEY;   //这就是全局最新的hash索引。
    while let Some(block) = blockheader_map.get(&header_hash) {
        //println!("block: {:?}", block);
        header_hash = block.headerhash;   //16进制输出

    }

//**********************************         准备工作          ******************************************/
    
    
    //以太坊遍历：从树根，遍历四颗树的所有叶子节点，  成功，不使用MAP
    let start1 = Instant::now(); // 开始计时  
    for (roothash, memdb) in root_hashes.iter().zip(memdbs.iter()) {
        let trie = EthTrie::new(memdb.clone()).at_root((*roothash).into());
        for (key, value) in trie.iter() {
            let FHP = &value[value.len() - 32..];
            //println!("Key: {:?}, Value: {:?}", hex::encode(key), hex::encode(value));
            //println!("FHP: {:?}", FHP);
        }
    }
    
    
    
    
    /*
已实现的，简化查询过程：
① 以太坊（略快）：遍历一次区块头，没有查BF，查询100次MPT树。 
缺少：根据100个地址查区块头的时间，和检测BF的时间。
后续弥补：遍历交易数组中的交易，查询区块头数组中的BF，并判断为真时。  累加。
*/
    //成功，弥补完成
    for transaction in &transactions {
        let address = &transaction.From; // 提取From地址
        // 遍历block_headers数组，使用每个区块头的布隆过滤器进行检查
        for block_header in &block_headers {
            let bf = &block_header.bloom;
            // 调用check_address_in_bloom函数检查地址是否可能存在于布隆过滤器中
            if check_address_in_bloom(bf, address) {
                //println!("地址 {:?} 可能存在于布隆过滤器中", hex::encode(address));
                break;
            } else {
                //println!("地址 {:?} 不在布隆过滤器中", hex::encode(address));
            }
        }
    }
    
    let duration1 = start1.elapsed(); // 计算经过的时间
    println!("区块链，链式查询时间为: {:?}", duration1);
    
    /*
新索引（略快）：100笔建立索引。 
 缺少：10次查询区块头，查询BF，判断为真。查询MPT的时间。
后续弥补：1.利用交易数组和count建立10个索引起始位置。 2. 查询10次区块头，查询BF，判断为真后，在MPT中查询到具体交易。即:10次查询MPT的时间

*/
    //将每个轮次的最后一笔交易组成一个数组
    let every_final_transaction: Vec<&Transaction> = transactions
        .iter() // 获取迭代器
        .skip(count - 1) // 跳过前count-1个元素，确保从第count个元素开始
        .step_by(count) // 之后每隔count-1个元素取一个
        .collect();
    //println!("Resulting transactions count: {}", every_final_transaction.len());
    
    
    let start2 = Instant::now(); // 开始计时  
    //查询到区块头，用BF判断是否查询到
    for transaction in &every_final_transaction {
        let address = &transaction.From; // 提取From地址
        // 遍历block_headers数组，使用每个区块头的布隆过滤器进行检查
        for block_header in &block_headers {
            let bf = &block_header.bloom;
            // 调用check_address_in_bloom函数检查地址是否可能存在于布隆过滤器中
            if check_address_in_bloom(bf, address) {
                //println!("地址 {:?} 可能存在于布隆过滤器中", hex::encode(address));
                break;
            } else {
                //println!("地址 {:?} 不在布隆过滤器中", hex::encode(address));
            }
        }
    }
    
    // 查询到区块头后，查询MPT，查询次数：every_final_transaction.len()， 成功
    for ((transaction, roothash), memdb) in every_final_transaction.iter().zip(root_hashes.iter()).zip(memdbs.iter()) {
    // 对每笔交易计算出用于MPT查找的key
        let mut hasher = Sha256::new();
        let value = concatenate_transaction_data(transaction); // 这些数据计算出key在MPT中查找
        hasher.update(&value);
        let hash = hasher.finalize(); // key

        // 使用对应的roothash和memdb创建Trie实例并查询
        let trie = EthTrie::new(memdb.clone()).at_root((*roothash).into());
        let v = trie.get(&hash);
        //println!("查询结果为：{:?} ", v);
        }
    
        // 不遍历MPT，使用hash指针从map中追溯并输出所有交易。   
        
        let mut current_hash = prev_hash;   //这就是全局最新的hash索引。
        while let Some(transaction) = total_map.get(&current_hash) {
            //println!("{:?}", transaction.FHP);
            //println!("{}", hex::encode(transaction.FHP));
            current_hash = transaction.FHP;   //16进制输出
            if current_hash.iter().all(|&b| b == 0) {
                break; // 当到达初始哈希值时停止
            }
        }
        let duration2 = start2.elapsed(); // 计算经过的时间
        println!("构建新索引后的查询时间为: {:?}", duration2);
}


