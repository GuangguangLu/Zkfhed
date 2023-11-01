use std::fmt::Debug;
use core::ops::Rem;
use num_traits::identities::Zero;
use polynomial_ring::Polynomial;
use rand::{Rng, thread_rng, SeedableRng};
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::rngs::StdRng;

pub fn main() {

    
    println!(" -------------------- Ⅰ ：Initialization BGV  -------------------------：");
    const N: i64 = 16;
    const t: i64 = 33554432;     
    const q: i64 = 1073741824;      
    
   
    let seed: [u8; 32] = [1; 32];
    
    let q_poly: Polynomial<i64>  = Polynomial::new(vec![1073741824]);

    let t_poly: Polynomial<i64> = Polynomial::new(vec![33554432]);
    
    
    let mut sk_v = vec![0; N as usize];  
    let mut rng = rand::rngs::StdRng::from_seed(seed);
    for i in 0..sk_v.len() {
        sk_v[i] = match rng.gen_range(0..3) {
            0 => -1,
            1 => 0,
            _ => 1,
        };
    }
    
    // Generate sk
    let sk: Polynomial<i64> = Polynomial::new(sk_v);

    
    // Generate pk1
    let mut pk1_v = vec![0; N as usize];
    let mut rng = rand::rngs::StdRng::from_seed(seed);
    for i in 0..pk1_v.len() {
        if i % 3 == 0 {
            pk1_v[i] = rng.gen_range(0..q-2);
        }
        if i % 5 == 1 {
        pk1_v[i] = -rng.gen_range(0..q-2);
        }
    }
    
    let pk1: Polynomial<i64> = Polynomial::new(pk1_v);

    //  Generate polynomial modulus
    let mut poly_mod_v = vec![0; N as usize + 1];
    poly_mod_v[0] = 1;
    poly_mod_v[N as usize] = 1;
    let poly_mod: Polynomial<i64> = Polynomial::new(poly_mod_v);
 
    
    // Generate pk0 = -a * pk1
    let r = -(&pk1 * &sk);  
    let mut pk0 = r.clone();
    let shang = pk0.division(&poly_mod);


    let pk0_mod_q = pk0.coeffs().iter().map(|x| (x % q + q) % q).collect::<Vec<_>>();

    let mut pk0: Polynomial<i64> = Polynomial::new(pk0_mod_q);   

    
    //Generate u
    let mut u_v = vec![0; N as usize];
    let mut rng = rand::rngs::StdRng::from_seed(seed);
    for i in 0..u_v.len() {
        u_v[i] = match rng.gen_range(0..3) {
            0 => -1,
            1 => 0,
            _ => 1,
        };
    }
    let u: Polynomial<i64> = Polynomial::new(u_v);
    
    // Generate EK0 & EK1
    let sk2 = poly_mod_poly(sk.clone()*sk.clone(),poly_mod.clone());
    let pk1sk = poly_mod_poly(-pk1.clone()*sk.clone(),poly_mod.clone());
    let EK0: Polynomial<i64> = sk2 + pk1sk;
    let EK1: Polynomial<i64> = pk1.clone();

    println!(" -------------------- Ⅰ ：END -------------------------：");


    println!("--- Ⅱ ：Read the dataset and set the scale of the neural network----------------");

    // Read the dataset
    let (inputs, targets) = read_dataset::<i64>(pk0.clone(), pk1.clone(), u.clone(), poly_mod.clone(), q_poly.clone(),t_poly.clone(),sk.clone());
    
    // Set neural network scale
    let input_size = inputs[0].len();  
    let hidden_size = 16;
    let output_size = 1;
    let learning_rate = 1e-13;
   
    let mut nn = NeuralNetwork::new(input_size, hidden_size, output_size, learning_rate,pk0.clone(),pk1.clone(), u.clone(),poly_mod.clone(),q_poly.clone(),t_poly.clone(),EK0.clone(),EK1.clone(),sk.clone());

    println!("--- Ⅱ ：END----------------");

    println!(" Ⅲ ：Started Training-----------------------------------------------------");
    nn.train(&inputs, &targets,6,pk0.clone(),pk1.clone(), u.clone(),poly_mod.clone(),q_poly.clone(),t_poly.clone(),EK0.clone(),EK1.clone(),sk.clone());
    println!(" Ⅲ ： END-----------------------------------------------------");
    

    println!(" Ⅳ ：Test model accuracy-----------------------------------------------------");
    let accuracy = nn.test(&inputs, &targets,pk0.clone(),pk1.clone(),u.clone(),sk.clone(),poly_mod.clone(),q_poly.clone(),t_poly.clone(),EK0.clone(),EK1.clone());
    println!("Accuracy: {:.2}%", accuracy * 100.0);

    println!(" Ⅳ ：END-----------------------------------------------------");
    
}

struct NeuralNetwork {   
    input_size: usize,
    hidden_size: usize,  
    output_size: usize,  
    learning_rate: f64,
    weights_ih: Vec<Vec<CtStruct<i64>>>,
    weights_ho: Vec<Vec<CtStruct<i64>>>,
}




impl NeuralNetwork {

    // Initialize the neural network and homomorphic encrypt the neural network
    fn new(input_size: usize, hidden_size: usize, output_size: usize, learning_rate: f64,pk0: Polynomial<i64>,pk1: Polynomial<i64>, u: Polynomial<i64>,poly_mod: Polynomial<i64>,q_poly: Polynomial<i64>,t_poly: Polynomial<i64>,EK0: Polynomial<i64>,EK1: Polynomial<i64>,sk: Polynomial<i64>) -> Self {
    
    // Initialize the neural network
    let mut rng = StdRng::seed_from_u64(0);
    let input_size = 4;
    let hidden_size = 16;
    let output_size = 1;

    let mut a: Vec<Vec<i64>> = (0..input_size)
        .map(|_| {
            (0..hidden_size)
                .map(|_| {
                    let num = rng.gen_range(-10..10) as f64;
                    ((num * 1000.0).round() / 1000.0) as i64
                })
                .collect()
        })
        .collect();

    let mut b: Vec<Vec<i64>> = (0..hidden_size)
        .map(|_| {
            (0..output_size)
                .map(|_| {
                    let num = rng.gen_range(-10..10) as f64;
                    ((num * 1000.0).round() / 1000.0) as i64
                })
                .collect()
        })
        .collect();
    //println!("初始的模型信息为{:?}", a);
    //println!("初始的模型信息为{:?}", b);
        
    // Homomorphic encrypt the neural network
        let w0 = Polynomial::new(vec![1]);
        let ciperw0 = encrypt(w0,pk0.clone(),pk1.clone(),u.clone(),poly_mod.clone(), q_poly.clone());
        
        let mut weights_ih = vec![vec![ciperw0.clone(); hidden_size]; input_size];  
        let mut weights_ho = vec![vec![ciperw0.clone(); output_size]; hidden_size];
        
        for i in 0..input_size {
            for j in 0..hidden_size {
                let chushi_w = Polynomial::new(vec![a[i][j]]);
                let chushi_w_cipher = encrypt(chushi_w,pk0.clone(),pk1.clone(),u.clone(),poly_mod.clone(), q_poly.clone());
                weights_ih[i][j] = chushi_w_cipher;
            }
        }
        
        for i in 0..hidden_size {
            for j in 0..output_size {
                let chushi_w = Polynomial::new(vec![b[i][j]]);
                let chushi_w_cipher = encrypt(chushi_w,pk0.clone(),pk1.clone(),u.clone(),poly_mod.clone(), q_poly.clone());
                weights_ho[i][j] = chushi_w_cipher;
            }
        }
         
        
        NeuralNetwork {
            input_size,
            hidden_size,
            output_size,
            learning_rate,
            weights_ih,
            weights_ho,
        }
    }
    // Ciphertext State ： Neural network forward
    fn forward(&self, input: &Vec<CtStruct<i64>>, pk0: Polynomial<i64>,pk1: Polynomial<i64>,u: Polynomial<i64>,poly_mod: Polynomial<i64>,q_poly: Polynomial<i64>,t_poly: Polynomial<i64>,EK0: Polynomial<i64>,EK1: Polynomial<i64>,sk: Polynomial<i64>) -> Vec<CtStruct<i64>> { 
        //由于输入的input是一维的，所以要把它构造成二维的 √
        let hidden = &multiply_matrices(&vec![input.to_vec()], &self.weights_ih,pk0.clone(),pk1.clone(),u.clone(),poly_mod.clone(),q_poly.clone(),t_poly.clone(),EK0.clone(),EK1.clone(),sk.clone())[0];  
        
        let test = decrypt(hidden[0].clone(),sk.clone(),poly_mod.clone(),q_poly.clone(),t_poly.clone());
        //println!("测试 前向传播过程中的，hidden，能否解密成功:{:?}", test);   
        
        
        let output = &multiply_matrices(&vec![hidden.to_vec()], &self.weights_ho,pk0.clone(),pk1.clone(),u.clone(),poly_mod.clone(),q_poly.clone(),t_poly.clone(),EK0.clone(),EK1.clone(),sk.clone())[0];   
        //println!("outputoutputoutputoutput{:?}", output); 
        
        
        //测试output能否解密成功
        let a = decrypt(output[0].clone(),sk.clone(),poly_mod.clone(),q_poly.clone(),t_poly.clone());
        //println!("测试output能否解密成功:{:?}", a);   //
        // [Polynomial { coeff: [26451560, -29568197, -29268220, -20181169, -10465212, 31967160, 33094644, 19985200, -29082635, -10445662, 25745813, 16064601, 2276060, 7882928, 4222348, -27773581] }] }
        
        return output.to_vec();
        

      
    }
    
    // Ciphertext State ： Neural network backward
    fn backward(&mut self, input: &Vec<CtStruct<i64>>, target: CtStruct<i64>, output: &Vec<CtStruct<i64>>, pk0: Polynomial<i64>, pk1: Polynomial<i64>, u: Polynomial<i64>, poly_mod: Polynomial<i64>,q_poly: Polynomial<i64>,t_poly: Polynomial<i64>,EK0: Polynomial<i64>,EK1: Polynomial<i64>,sk: Polynomial<i64>) {
        
        // 计算输出层梯度
        let mut output_gradients: Vec<CtStruct<i64>> = Vec::new();     
        
        for i in 0..self.output_size {   

            let error = c_sub_c(output[i].clone(),target.clone(),q_poly.clone());
            
            output_gradients.push(error);
            //println!("密文的梯度信息为：{:?}", output_gradients);
        }
        // √
       
        let m_output_gradients = decrypt(output_gradients[0].clone(),sk.clone(),poly_mod.clone(),q_poly.clone(),t_poly.clone());
        //println!("明文的output_gradients，明文的error没乘学习率之前：{:?}", m_output_gradients);
        
        let a = m_output_gradients.polys[0].coeffs()[0]; 
        let a_float = a as f64; 
        let m_output_gradients_LR = a_float * self.learning_rate; 
        let z_m_output_gradients_LR = m_output_gradients_LR as i64;
        
        //println!("梯度信息为{:?}", z_m_output_gradients_LR); 
         
        let z_vec = vec![z_m_output_gradients_LR];  
        let z_m_output_gradients_LR_poly = Polynomial::new(z_vec);   
        let c_output_gradients_LR = encrypt(z_m_output_gradients_LR_poly.clone(),pk0.clone(),pk1.clone(),u.clone(),poly_mod.clone(), q_poly.clone());   
        
        // 更新隐藏层到输出层的权重
        let hidden = &multiply_matrices(&vec![input.to_vec()], &self.weights_ih,pk0.clone(),pk1.clone(),u.clone(),poly_mod.clone(),q_poly.clone(),t_poly.clone(),EK0.clone(),EK1.clone(),sk.clone())[0];
        //println!("hiddenhiddenhiddenhiddenhiddenhidden{:?}", hidden);

        for i in 0..self.hidden_size { 
            for j in 0..self.output_size { 
                
                let delta_w = c_mul_c(c_output_gradients_LR.clone(),hidden[i].clone(),poly_mod.clone(),q_poly.clone(),EK0.clone(),EK1.clone());
                
                let a = c_sub_c(self.weights_ho[i][j].clone(),delta_w.clone(),q_poly.clone()); 
    
                self.weights_ho[i][j] = a;
            }
        }
        
        // 计算隐藏层梯度 
        let mut hidden_gradients: Vec<CtStruct<i64>> = Vec::new();
        for i in 0..self.hidden_size { 
            
            let error = c_mul_c(c_output_gradients_LR.clone(),self.weights_ho[i][0].clone(),poly_mod.clone(),q_poly.clone(),EK0.clone(),EK1.clone());
            
            hidden_gradients.push(error);
        }
    
        // 更新输入层到隐藏层的权重
        for i in 0..self.input_size {
            for j in 0..self.hidden_size {   
                
                let delta_w = c_mul_c(hidden_gradients[j].clone(),input[i].clone(),poly_mod.clone(),q_poly.clone(),EK0.clone(),EK1.clone());

                let a = c_sub_c(self.weights_ih[i][j].clone(),delta_w.clone(),q_poly.clone()); 
                self.weights_ih[i][j] = a;
                }
        }
        

    }   // 反向传播结束
    
    // ciphertext state ： Neural Network Training
    fn train(&mut self, inputs: &Vec<Vec<CtStruct<i64>>>, targets: &Vec<CtStruct<i64>> ,epochs: i64,pk0: Polynomial<i64>,pk1: Polynomial<i64>, u: Polynomial<i64>,poly_mod: Polynomial<i64>,q_poly: Polynomial<i64>,t_poly: Polynomial<i64>,EK0: Polynomial<i64>,EK1: Polynomial<i64>,sk: Polynomial<i64>) {
        
        for i in 0..epochs {
            eprintln!("第{:?}次模型训练开始", i);
            let mut total_loss = 0;
            for (input, target) in inputs.iter().zip(targets.iter()) { 
                let output = self.forward(input,pk0.clone(),pk1.clone(),u.clone(),poly_mod.clone(),q_poly.clone(),t_poly.clone(),EK0.clone(),EK1.clone(),sk.clone());    
                eprintln!("第{:?}次前向传播完成", i);
                let ming = decrypt(output[0].clone(),sk.clone(),poly_mod.clone(),q_poly.clone(),t_poly.clone());
                //eprintln!("此次前向传播，的output结果为{:?}", ming);

                let loss = c_sub_c(target.clone(),output[0].clone(),q_poly.clone());
                let ming_loss = decrypt(loss.clone(),sk.clone(),poly_mod.clone(),q_poly.clone(),t_poly.clone());
                let a = ming_loss.polys[0].coeffs()[0];
               
                total_loss += a;          
                //eprintln!("total_loss是否变化{:?}", total_loss);
                self.backward(input, target.clone(), &output,pk0.clone(), pk1.clone(), u.clone(),poly_mod.clone(),q_poly.clone(),t_poly.clone(),EK0.clone(),EK1.clone(),sk.clone());  
                //eprintln!("第{:?}次模型的反向传播完成", i);
            }
            eprintln!("total_loss: {}",total_loss / 100);
        
            //println!("每个epoch后，模型weights_ih更新结果为：--------------------{:?}", self.weights_ih);
        }
    }
    // √
    
    // Test model accuracy
    fn test(&self, inputs: &Vec<Vec<CtStruct<i64>>>, targets: &Vec<CtStruct<i64>>,pk0: Polynomial<i64>,pk1: Polynomial<i64>,u: Polynomial<i64>,sk: Polynomial<i64>,poly_mod: Polynomial<i64>,q_poly: Polynomial<i64>,t_poly: Polynomial<i64>,EK0: Polynomial<i64>,EK1: Polynomial<i64>) -> f64 {
        let mut correct_predictions = 0;
        
        for (i, (input, target)) in inputs.iter().zip(targets.iter()).enumerate() {
           
            
            let output = self.forward(input,pk0.clone(),pk1.clone(),u.clone(),poly_mod.clone(),q_poly.clone(),t_poly.clone(),EK0.clone(),EK1.clone(),sk.clone());
            let ming_output = decrypt(output[0].clone(),sk.clone(),poly_mod.clone(),q_poly.clone(),t_poly.clone());
            let ming_target = decrypt(target.clone(),sk.clone(),poly_mod.clone(),q_poly.clone(),t_poly.clone());
            
            let mut b = 0;
            let C = ming_target.polys[0].coeffs();
            
            if C.is_empty() {
                b = 0;
            } else {
                b = ming_target.polys[0].coeffs()[0];
            }
            
            let mut a = ming_output.polys[0].coeffs()[0];
            
            if (a < 5 && b == 0) {   
                correct_predictions += 1;
            } 

            if (a > 5 && b == 10) {   
                correct_predictions += 1;
            } 
            
        
        }
        
        return correct_predictions as f64 / targets.len() as f64;
    }
}


// Ciphertext matrix multiplication function
fn multiply_matrices(a: &Vec<Vec<CtStruct<i64>>>, b: &Vec<Vec<CtStruct<i64>>>,pk0: Polynomial<i64>,pk1: Polynomial<i64>,u: Polynomial<i64>,poly_mod: Polynomial<i64>,q_poly: Polynomial<i64>,t_poly: Polynomial<i64>,EK0: Polynomial<i64>,EK1: Polynomial<i64>,sk: Polynomial<i64>) -> Vec<Vec<CtStruct<i64>>> {
    
    assert!(a[0].len() == b.len(), "矩阵：A and B can't be MULTIPLIED");

    let mut result: Vec<Vec<CtStruct<i64>>> = vec![vec![CtStruct { polys: Vec::new() }; b[0].len()]; a.len()]; // ↑测试通过↓
    let ling: Polynomial<i64> = Polynomial::new(vec![0]);
    let ling_cipher = encrypt(ling,pk0.clone(),pk1.clone(),u.clone(),poly_mod.clone(), q_poly.clone());
    
    for i in 0..a.len() { 
        for j in 0..b[0].len() { 
            let mut add1 = ling_cipher.clone();
            for k in 0..b.len() {  
                //println!("a[i][k] ，密文1为：------------：{:?}", a[i][k]); 
                //println!("b[k][j] ，密文1为：------------：{:?}", b[k][j]);
                
                
                let mut mid_result = c_mul_c(a[i][k].clone(),b[k][j].clone(),poly_mod.clone(),q_poly.clone(),EK0.clone(),EK1.clone());
                                
                add1 = c_add_c(add1.clone(),mid_result.clone(),q_poly.clone());    

            }
            result[i][j] = add1;
        }
    }

    return result;
    
}

// Read the dataset and homomorphic encrypt each value
fn read_dataset<T: std::clone::Clone+ for<'x> std::ops::SubAssign<&'x T>+ for<'x> std::ops::AddAssign<&'x T>+ for<'x> std::ops::MulAssign<&'x T>+ for<'x> std::ops::DivAssign<&'x T>+ num_traits::identities::Zero + std::fmt::Debug>(pk0: Polynomial<i64>, pk1: Polynomial<i64>, u: Polynomial<i64>, poly_mod: Polynomial<i64>, q_poly: Polynomial<i64>,t_poly: Polynomial<i64>,sk: Polynomial<i64>) -> (Vec<Vec<CtStruct<i64>>>,Vec<CtStruct<i64>>){
    let mut c_data: Vec<Vec<CtStruct<i64>>> = Vec::new();
    let mut c_labels: Vec<CtStruct<i64>> = Vec::new();

    
    let file = File::open("wdbc.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut arr = Vec::new();
    let mut j = 0;  
    
    for (row_idx, line) in reader.lines().enumerate() {  

        arr.clear();
        
        
        for item in line.expect("Failed to read line").split(',') {
            let number: i64 = item
                .trim()
                .parse()
                .expect(&format!("Failed to parse number: {}", item));
            arr.push(number);
        }

        
        for i in 0..arr.len(){ 
            if i == arr.len()-1{
                 let v = vec![arr[i]];
                 let m_poly = Polynomial::new(v);
                 let c = encrypt(m_poly,pk0.clone(),pk1.clone(),u.clone(),poly_mod.clone(), q_poly.clone());
                 c_labels.push(c);                
            }
            else if i == 0{   
                c_data.push(Vec::new()); 
                let v = vec![arr[i]];         
                let m_poly = Polynomial::new(v);
                let c = encrypt(m_poly,pk0.clone(),pk1.clone(),u.clone(),poly_mod.clone(), q_poly.clone());
          
                c_data[j].push(c);
            }
            
            else{  
                 let v = vec![arr[i]];         
                 let m_poly = Polynomial::new(v);
                 let c = encrypt(m_poly,pk0.clone(),pk1.clone(),u.clone(),poly_mod.clone(), q_poly.clone());
          
                 c_data[j].push(c);
            }
        }
        j += 1;
    }  
    
    let a = decrypt(c_data[1][1].clone(),sk.clone(),poly_mod.clone(),q_poly.clone(),t_poly.clone());
    //println!("测试数据集是否加密，解密成功：{:?}", a);  
    
    return (c_data,c_labels);
}


#[derive(Clone)]
#[derive(Debug)]
// Ciphertext structure
pub struct CtStruct<T> {  
    polys: Vec<Polynomial<T>>,
}

// Function: Polynomial mod Polynomial
pub fn poly_mod_poly<T: std::clone::Clone+ for<'x> std::ops::SubAssign<&'x T>+ for<'x> std::ops::AddAssign<&'x T>+ for<'x> std::ops::MulAssign<&'x T>+ for<'x> std::ops::DivAssign<&'x T>+ num_traits::identities::Zero >(poly1: Polynomial<T>, poly2: Polynomial<T>) -> Polynomial<T>{
    
    let mut yushu = poly1.clone();
    let mut shang = yushu.division(&poly2);
    return yushu;
    
}

// Function: Polynomial mod integer，q
pub fn poly_mod_q<T: std::clone::Clone+ for<'x> std::ops::SubAssign<&'x T>+ for<'x> std::ops::AddAssign<&'x T>+ for<'x> std::ops::MulAssign<&'x T>+ for<'x> std::ops::DivAssign<&'x T>+ num_traits::identities::Zero + std::fmt::Debug>(ct: CtStruct<T>, q_poly:Polynomial<T> ) -> CtStruct<T>{
    let mut out_cipher = CtStruct { polys: Vec::new() };
    let LL = ct.polys.len();

    for i in 0..LL{

        let mut shang = ct.polys[i].clone().division(&q_poly);
        
        let mulmul = q_poly.clone()*shang;

        out_cipher.polys.push(ct.polys[i].clone()-mulmul);
    }
    return out_cipher;

    
}

// Function: Decryption of BGV
pub fn decrypt<T: std::clone::Clone+ for<'x> std::ops::SubAssign<&'x T>+ for<'x> std::ops::AddAssign<&'x T>+ for<'x> std::ops::MulAssign<&'x T>+ for<'x> std::ops::DivAssign<&'x T>+ num_traits::identities::Zero+std::fmt::Debug + std::cmp::PartialOrd>(ct: CtStruct<T>, sk: Polynomial<T>, g: Polynomial<T>,q_poly: Polynomial<T>, t_poly: Polynomial<T>) -> CtStruct<T> {
    
    let mut out = CtStruct { polys: Vec::new() };
    out.polys.push(poly_mod_poly(ct.polys[0].clone(), g.clone()));
    out.polys[0] += poly_mod_poly(ct.polys[1].clone() * sk.clone(), g.clone())  ;  
    out = poly_mod_q(out,q_poly.clone());  // mod q
    out = poly_mod_q(out,t_poly.clone());  // mod t

    return out;

}

// Function: Encryption of BGV
pub fn encrypt<T: std::clone::Clone+ for<'x> std::ops::SubAssign<&'x T>+ for<'x> std::ops::AddAssign<&'x T>+ for<'x> std::ops::MulAssign<&'x T>+ for<'x> std::ops::DivAssign<&'x T>+ num_traits::identities::Zero + std::fmt::Debug>(m_poly: Polynomial<T>, pk0: Polynomial<T>, pk1: Polynomial<T>, u: Polynomial<T>, g: Polynomial<T>, q_poly: Polynomial<T>) -> CtStruct<T> {
    let ct0 = m_poly + poly_mod_poly(&pk0 * &u, g.clone());
    let ct1 = poly_mod_poly(&pk1 * &u, g.clone());
    
    let mut my_struct = CtStruct { polys: Vec::new() };  
    
    my_struct.polys.push(ct0.clone());  
    my_struct.polys.push(ct1.clone()); 
    
    my_struct = poly_mod_q(my_struct.clone(),q_poly.clone());  
    return my_struct;
}

// Function: Homomorphic addition of BGV
pub fn c_add_c<T: std::clone::Clone+ for<'x> std::ops::SubAssign<&'x T>+ for<'x> std::ops::AddAssign<&'x T>+ for<'x> std::ops::MulAssign<&'x T>+ for<'x> std::ops::DivAssign<&'x T>+ num_traits::identities::Zero + std::fmt::Debug + std::ops::Rem<i64, Output = T>>(ct0: CtStruct<T>,ct1: CtStruct<T>, q_poly: Polynomial<T>) -> CtStruct<T>  where <T as Rem<i64>>::Output: Debug, <T as Rem<i64>>::Output: num_traits::Zero {
    // 密文长度不同时候，也是对应项，相加
    let mut out_cipher = ct0; 
    for i in 0..2 {
        out_cipher.polys[i] += &ct1.polys[i];
    }
    
    //let out_cipher = poly_mod_q(out_cipher,q_poly.clone());
    return out_cipher;

}

// Function: Homomorphic subtraction of BGV
pub fn c_sub_c<T: std::clone::Clone+ for<'x> std::ops::SubAssign<&'x T>+ for<'x> std::ops::AddAssign<&'x T>+ for<'x> std::ops::MulAssign<&'x T>+ for<'x> std::ops::DivAssign<&'x T>+ num_traits::identities::Zero + std::fmt::Debug + std::ops::Rem<i64, Output = T>>(ct0: CtStruct<T>,ct1: CtStruct<T>, q_poly: Polynomial<T>) -> CtStruct<T>  where <T as Rem<i64>>::Output: Debug, <T as Rem<i64>>::Output: num_traits::Zero {
    
    let mut out_cipher = ct0; 
    for i in 0..2 {
        out_cipher.polys[i] -= &ct1.polys[i];
    }
    
    //let out_cipher = poly_mod_q(out_cipher,q_poly.clone());
    return out_cipher;

}

pub fn c_div_q<T: std::clone::Clone+ for<'x> std::ops::SubAssign<&'x T>+ for<'x> std::ops::AddAssign<&'x T>+ for<'x> std::ops::MulAssign<&'x T>+ for<'x> std::ops::DivAssign<&'x T>+ num_traits::identities::Zero + std::fmt::Debug + std::ops::Rem<i64, Output = T>>(ct0: CtStruct<i64>,q: i64) -> CtStruct<i64>  where <T as Rem<i64>>::Output: Debug, <T as Rem<i64>>::Output: num_traits::Zero {


    let mut out_cipher = ct0; 
    
    let Q_poly: Polynomial<i64> = Polynomial::new(vec![q]);
    for i in 0..2 {
        out_cipher.polys[i] = out_cipher.polys[i].clone().division(&Q_poly.clone());
    }
    
    return out_cipher;

}

// Function: Ciphertext multiplication integer
pub fn c_mul_q<T: std::clone::Clone+ for<'x> std::ops::SubAssign<&'x T>+ for<'x> std::ops::AddAssign<&'x T>+ for<'x> std::ops::MulAssign<&'x T>+ for<'x> std::ops::DivAssign<&'x T>+ num_traits::identities::Zero + std::fmt::Debug + std::ops::Rem<i64, Output = T>>(ct0: CtStruct<i64>,q: i64) -> CtStruct<i64>  where <T as Rem<i64>>::Output: Debug, <T as Rem<i64>>::Output: num_traits::Zero {

    let mut out_cipher = ct0; 
    
    for i in 0..2 {
        out_cipher.polys[i] = out_cipher.polys[i].clone() * q;
    }
    
    return out_cipher;

}

// Function: Homomorphic multiplication of BGV
pub fn c_mul_c<T: std::clone::Clone+ for<'x> std::ops::SubAssign<&'x T>+ for<'x> std::ops::AddAssign<&'x T>+ for<'x> std::ops::MulAssign<&'x T>+ for<'x> std::ops::DivAssign<&'x T>+ num_traits::identities::Zero + std::fmt::Debug + std::ops::Rem<i64, Output = T>>(ct0: CtStruct<T>,ct1: CtStruct<T>, poly_mod: Polynomial<T>,q_poly: Polynomial<T>,EK0: Polynomial<T>,EK1: Polynomial<T>) -> CtStruct<T> {
    
    let mut out_cipher = CtStruct { polys: Vec::new() }; 
    for i in 0..ct0.polys.len(){
        for j in 0..ct1.polys.len(){
            if i+j >= out_cipher.polys.len(){
                out_cipher.polys.push(poly_mod_poly(ct0.polys[i].clone()*ct1.polys[j].clone(),poly_mod.clone()))   
            }
            else{
                out_cipher.polys[i+j] += poly_mod_poly(ct0.polys[i].clone()*ct1.polys[j].clone(),poly_mod.clone());
            }
        }
    }
    //println!(" -------------Simple Implementation of Key Switching -------------------------------------------：");
    let rec1 = out_cipher.polys[0].clone() + poly_mod_poly(EK0.clone()*out_cipher.polys[2].clone(),poly_mod.clone());
    let rec2 = out_cipher.polys[1].clone() + poly_mod_poly(EK1.clone()*out_cipher.polys[2].clone(),poly_mod.clone());
    
    let mut c1c2 = CtStruct { polys: Vec::new() }; 
    c1c2.polys.push(rec1.clone());
    c1c2.polys.push(rec2.clone());
    //c1c2 = poly_mod_q(c1c2,q_poly.clone());

    return c1c2;

}