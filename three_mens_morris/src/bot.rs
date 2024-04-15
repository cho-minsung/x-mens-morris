extern crate rand;

use rand::Rng;

pub struct Neuron {
    weights: Vec<f32>,
    bias: f32,
}

pub struct Layer {
    neurons: Vec<Neuron>,
}

impl Neuron {
    pub fn new(num_inputs: usize) -> Self {
        let mut rng = rand::thread_rng();
        let weights: Vec<f32> = (0..num_inputs).map(|_| rng.gen::<f32>()).collect();
        let bias = rng.gen::<f32>();

        Neuron { weights, bias }
    }

    pub fn weights(&self) -> &Vec<f32> {
        &self.weights
    }

    pub fn bias(&self) -> f32 {
        self.bias
    }

    // Function to multiply inputs by weights and add bias
    pub fn linear_combination(&self, inputs: &Vec<f32>) -> f32 {
        let sum: f32 = self.weights.iter().zip(inputs).map(|(w, i)| w * i).sum();
        sum + self.bias
    }

     // Activation function using ReLU
     pub fn relu(&self, inputs: &Vec<f32>) -> f32 {
        let linear_output = self.linear_combination(inputs);
        if linear_output > 0.0 {
            linear_output
        } else {
            0.0
        }
    }

    pub fn softmax(&self, inputs: &Vec<f32>) -> Vec<f32> {
        let exps: Vec<f32> = inputs.iter().map(|x| f32::exp(*x)).collect();
        let sum_exps: f32 = exps.iter().sum();
        exps.iter().map(|x| x / sum_exps).collect()
    }
}

fn main() {
    
}
