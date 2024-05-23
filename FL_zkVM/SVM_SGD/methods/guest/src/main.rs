#![no_main]
// If you want to try std support, also update the guest Cargo.toml file



use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

struct DataPoint {
    features: Vec<f64>,
    label: f64,
}

fn sgd_train(data: &[DataPoint], learning_rate: f64, num_iterations: usize) -> Vec<f64> {
    let num_features = data[0].features.len();
    let mut weights = vec![0.0; num_features];

    for _ in 0..num_iterations {
        let random_index = rand::random::<usize>() % data.len();
        let point = &data[random_index];

        let predicted = dot_product(&weights, &point.features);
        let error = point.label - predicted;

        for i in 0..num_features {
            weights[i] += learning_rate * error * point.features[i];
        }
    }

    weights
}

fn predict(weights: &[f64], features: &[f64]) -> f64 {
    dot_product(weights, features)
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
    let weights = sgd_train(&data, learning_rate, num_iterations);

    // 对新数据进行预测
    let test_data = vec![
        vec![0.2, 0.4],
        vec![-0.1, -0.3],
        // 其他测试数据...
    ];

    for features in test_data {
        let prediction = predict(&weights, &features);
        println!("Predicted label: {}", prediction);
    }
}

fn dot_product(weights: &[f64], features: &[f64]) -> f64 {
    let mut result = 0.0;

    for i in 0..weights.len() {
        result += weights[i] * features[i];
    }

    result
}