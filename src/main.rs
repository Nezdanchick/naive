use naive::networks::*;
use naive::{Neuron, /*Synapse*/};

fn main() {
    let mut perceptron = Perceptron::new(vec![3, 2, 1]);
    println!("{}", perceptron);
    let neurons = &[Neuron::empty()];

    perceptron.input(neurons);
    println!("{}", perceptron);
}