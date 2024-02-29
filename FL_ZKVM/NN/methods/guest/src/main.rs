// 函数话 //
// polynomial-ring = "0.5.0"

#![no_main]




//改动： sigmoid里边x的符号，梯度计算时的学习率符号，损失函数，隐藏层大小
use risc0_zkvm::guest::env;
use rand::rngs::StdRng;
use rand::{thread_rng, Rng, SeedableRng};
use std::fs::File;
use std::io::{BufRead, BufReader};

risc0_zkvm::guest::entry!(main);

fn read_dataset() -> (Vec<Vec<f64>>, Vec<f64>) {

    let data: Vec<Vec<f64>> = vec![
            vec![51.0,35.0],
            vec![20.0,5.0],
            vec![51.0,35.0],
            vec![20.0,5.0],
            vec![51.0,35.0],
            vec![20.0,5.0],
            vec![51.0,35.0],
            vec![20.0,5.0],
            vec![51.0,35.0],
            vec![20.0,5.0],
            vec![51.0,35.0],
            vec![20.0,5.0]
        ];   

        let labels: Vec<f64> = vec![0.0,1.0,0.0,1.0,0.0,1.0,0.0,1.0,0.0,1.0,0.0,1.0];
       
    (data, labels)
}

// 矩阵计算   √
fn multiply_matrices(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    assert!(a[0].len() == b.len(), "矩阵：A and B can't be MULTIPLIED");
    let mut result = vec![vec![0.0; b[0].len()]; a.len()]; //先写列，再写行。

    for i in 0..a.len() {
        //3
        for j in 0..b[0].len() {
            //1
            for k in 0..b.len() {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    result
}




struct NeuralNetwork {
    input_size: usize,
    hidden_size: usize,
    output_size: usize,
    learning_rate: f64,
    weights_ih: Vec<Vec<f64>>,
    weights_ho: Vec<Vec<f64>>,
}

impl NeuralNetwork {
    fn new(input_size: usize, hidden_size: usize, output_size: usize, learning_rate: f64) -> Self {
    let mut rng = StdRng::seed_from_u64(0);

    let weights_ih = (0..input_size)
        .map(|_| {
            (0..hidden_size)
                .map(|_| {
                    let num = rng.gen_range(-10..10) as f64 ;
                    (num * 1000.0).round() / 1000.0
                })
                .collect()
        })
.collect();
   
    let weights_ho = (0..hidden_size)
        .map(|_| {
            (0..output_size)
                .map(|_| {
                    let num = rng.gen_range(-10..10) as f64 ;
                    (num * 1000.0).round() / 1000.0
                })
                .collect()
        })
.collect();

        //let weights_ih = vec![vec![1.0; hidden_size]; input_size];  // i行，h列，初始值为0.0
        //let weights_ho = vec![vec![1.0; output_size]; hidden_size];
        println!("{:?}", weights_ih);
        println!("{:?}", weights_ho);

        NeuralNetwork {
            input_size,
            hidden_size,
            output_size,
            learning_rate,
            weights_ih,
            weights_ho,
        }
    }

    fn test(&self, inputs: &Vec<Vec<f64>>, targets: &Vec<f64>) -> f64 {
        let mut correct_predictions = 0;
        for (input, target) in inputs.iter().zip(targets.iter()) {
            let output = self.forward(input);
            
            if (output[0] > 5.0 && *target == 10.0) || (output[0] < 5.0 && *target == 0.0) {
                correct_predictions += 1;
            }
        }
        correct_predictions as f64 / targets.len() as f64
    }

    fn forward(&self, input: &Vec<f64>) -> Vec<f64> {
        let hidden = &multiply_matrices(&vec![input.clone()], &self.weights_ih)[0];
        let output = &multiply_matrices(&vec![hidden.clone()], &self.weights_ho)[0];


        output.to_vec()
    } 

    fn backward(&mut self, input: &Vec<f64>, target: f64, output: &Vec<f64>) {
        // 计算输出层梯度
        let mut output_gradients = Vec::new();
        for i in 0..self.output_size {
            // let error = -2.0 * (target - output[i]); 
            let error = output[i] - target;
            let gradient = error; 
            output_gradients.push(gradient); 
        }

        // 更新隐藏层到输出层的权重
        let hidden = &multiply_matrices(&vec![input.clone()], &self.weights_ih)[0]; 

        for i in 0..self.hidden_size {
            for j in 0..self.output_size {
                let delta_w = self.learning_rate * output_gradients[j] * hidden[i]; 
                self.weights_ho[i][j] -= delta_w;
            }
        }

        // 计算隐藏层梯度
        let mut hidden_gradients = Vec::new(); 
        for i in 0..self.hidden_size {
            let mut error = 0.0;
            for j in 0..self.output_size {
                error += output_gradients[j] * self.weights_ho[i][j]; 
            } 
            let gradient = error; 
            hidden_gradients.push(gradient); 
        }

        // 更新输入层到隐藏层的权重
        for i in 0..self.input_size {
            for j in 0..self.hidden_size {
                let delta_w = self.learning_rate * hidden_gradients[j] * input[i]; 
                self.weights_ih[i][j] -= delta_w;
            }
        }
    }

    fn train(&mut self, inputs: &Vec<Vec<f64>>, targets: &Vec<f64>, epochs: u32) {
        for epoch in 0..epochs {
            let mut total_loss = 0.0;
            for (input, target) in inputs.iter().zip(targets.iter()) {
                let output = self.forward(input);
                //println!("outputoutputoutput{:?}", output);
                let loss = 0.5 * (*target - output[0]).powi(2);  
                //let loss: f64 = -(target * output[0].log2() + (1.0-target) * (1.0 - output[0]).log2());  //二元交叉熵损失函数
                total_loss += loss;
                self.backward(input, *target, &output);
            }
            // println!("weights_ih--------------------{:?}", self.weights_ih);
            println!(
                "Epoch: {}, Loss: {}",
                epoch,
                total_loss / (inputs.len() as f64)
            );
        }
    }
}

fn main() {
    let (inputs, targets) = read_dataset();
    let input_size = inputs[0].len();
    let hidden_size = 2;
    let output_size = 1;
    let learning_rate = 1e-7;

    let mut nn = NeuralNetwork::new(input_size, hidden_size, output_size, learning_rate);
    nn.train(&inputs, &targets, 10);
    //println!("训练结束后模型信息为：{:?}%", nn.weights_ih);
    
    //let accuracy = nn.test(&inputs, &targets);
    
    //println!("Accuracy: {:.2}%", accuracy * 100.0);
}
