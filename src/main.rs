use rand::prelude::*;
use rand::thread_rng;
use textplots::{Chart, Plot, Shape};

// Function to generate training data
fn generate_data(num_samples: usize) -> (Vec<f32>, Vec<f32>) {
    let mut rng = thread_rng();
    let mut x_data = Vec::new();
    let mut y_data = Vec::new();

    for _ in 0..num_samples {
        let x: f32 = rng.gen_range(-10.0..10.0);
        let noise: f32 = rng.gen_range(-1.0..1.0);
        let y: f32 = 2.0 * x + 1.0 + noise; // y = 2x + 1 + noise
        x_data.push(x);
        y_data.push(y);
    }

    (x_data, y_data)
}

// Define Linear Regression Model
#[derive(Debug)]
struct LinearRegression {
    weight: f32,
    bias: f32,
}

impl LinearRegression {
    fn new() -> Self {
        Self { weight: 0.0, bias: 0.0 }
    }

    fn forward(&self, x: f32) -> f32 {
        self.weight * x + self.bias
    }

    fn train(&mut self, x_data: &[f32], y_data: &[f32], epochs: usize, learning_rate: f32) {
        for epoch in 0..epochs {
            let mut total_loss = 0.0;

            for (&x, &y) in x_data.iter().zip(y_data.iter()) {
                let prediction = self.forward(x);
                let error = prediction - y;

                // Gradient descent update
                self.weight -= learning_rate * 2.0 * error * x;
                self.bias -= learning_rate * 2.0 * error;

                total_loss += error * error;
            }

            // Print loss every 100 epochs
            if epoch % 100 == 0 {
                println!("Epoch {}: Loss = {:.6}", epoch, total_loss / x_data.len() as f32);
            }
        }
    }
}

// Function to plot data
fn plot_data(title: &str, x_data: &[f32], y_data: &[f32]) {
    let points: Vec<(f32, f32)> = x_data.iter().zip(y_data.iter()).map(|(&x, &y)| (x, y)).collect();

    println!("\n{}", title);
    Chart::new(100, 30, -10.0, 10.0)
        .lineplot(&Shape::Points(&points))
        .display();
}

fn main() {
    let (x_data, y_data) = generate_data(100);
    let mut model = LinearRegression::new();

    println!("Training the model...");
    model.train(&x_data, &y_data, 1000, 0.001);

    println!("\nTrained Model: {:?}", model);

    // Test the trained model
    let test_x = 5.0;
    let predicted_y = model.forward(test_x);
    println!("For x = {}, predicted y = {:.6}", test_x, predicted_y);

    // Plot actual vs predicted data
    plot_data("Actual Data:", &x_data, &y_data);

    let predicted_y_data: Vec<f32> = x_data.iter().map(|&x| model.forward(x)).collect();
    plot_data("Predicted Data:", &x_data, &predicted_y_data);
}