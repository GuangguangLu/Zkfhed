#![no_main]
// If you want to try std support, also update the guest Cargo.toml file



use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

struct DataPoint {
    features: Vec<f64>,
    label: f64,
}



fn main() {
    // 创建一些示例数据点
    let data = vec![
        DataPoint { features: vec![0.5, 0.8], label: 1.0 },
        DataPoint { features: vec![-0.3, -0.6], label: -1.0 },
        // 其他数据点...
    ];

    // 训练模型
    let learning_rate = 0.01;
    let num_iterations = 100;
    let weights = train_svm(&data, learning_rate, num_iterations);

    // 对新数据进行预测
    let test_data = vec![
        vec![0.2, 0.4],
        vec![-0.1, -0.3],
        // 其他测试数据...
    ];
/*
    for features in test_data {
        let prediction = predict(&weights, &features);
        println!("Predicted label: {}", prediction);
    }
*/
}

fn dot_product(weights: &[f64], features: &[f64]) -> f64 {
    let mut result = 0.0;

    for i in 0..weights.len() {
        result += weights[i] * features[i];
    }

    result
}

use rand::Rng;

fn train_svm(data: &[DataPoint], learning_rate: f64, num_iterations: usize) -> Vec<f64> {
    let num_features = data[0].features.len();
    let mut weights = vec![0.0; num_features];

    let epsilon = 1.0; // 控制差分隐私的隐私预算
    let sensitivity = calculate_sensitivity(&data); // 计算敏感度

    for _ in 0..num_iterations {
        for point in data {
            let predicted = dot_product(&weights, &point.features);
            let delta = learning_rate * (point.label as f64 - predicted);
            for i in 0..num_features {
                let noise = laplace_noise(epsilon / num_iterations as f64, sensitivity);
                weights[i] += delta * point.features[i] + noise;
            }
        }
    }

    weights
}
/*
fn predict(weights: &[f64], features: &[f64]) -> i32 {
    let threshold = 0.0;
    let predicted = dot_product(weights, features);
    let noisy_prediction = add_noise(predicted, sensitivity);
    if noisy_prediction >= threshold {
        1
    } else {
        -1
    }
}
*/

fn calculate_sensitivity(data: &[DataPoint]) -> f64 {
    let mut max_sensitivity = 0.0;
    for point1 in data {
        for point2 in data {
            let sensitivity = calculate_point_sensitivity(&point1, &point2);
            if sensitivity > max_sensitivity {
                max_sensitivity = sensitivity;
            }
        }
    }
    max_sensitivity
}

fn calculate_point_sensitivity(point1: &DataPoint, point2: &DataPoint) -> f64 {
    let mut sensitivity = 0.0;
    for i in 0..point1.features.len() {
        let diff = (point1.features[i] - point2.features[i]).abs();
        if diff > sensitivity {
            sensitivity = diff;
        }
    }
    sensitivity
}

fn laplace_noise(epsilon: f64, sensitivity: f64) -> f64 {
    let scale = sensitivity / epsilon;
    let mut rng = rand::thread_rng();
    let uniform = rng.gen::<f64>() - 0.5;
    let noise = scale * uniform.signum() * (-uniform.abs().ln());
    noise
}

fn add_noise(value: f64, sensitivity: f64) -> f64 {
    let epsilon = 1.0; // 控制差分隐私的隐私预算
    let noise = laplace_noise(epsilon, sensitivity);
    value + noise
}