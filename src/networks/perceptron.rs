use crate::networks::neural_network::*;
use crate::neuron::Neuron;
use std::fmt::Display;
pub struct Perceptron {
    _slices: Vec<usize>,
    _neurons: Vec<Neuron>
}
impl Perceptron {
    pub fn new(slices: Vec<usize>) -> Perceptron {
        Perceptron { 
            _slices: slices.clone(), _neurons: Vec::new()
        }
    }
}
impl NeuralNetwork for Perceptron {
    #[allow(unused_variables)]
    fn input(&mut self, neurons: &[Neuron]) {
        
    }
    #[allow(unused_variables)]
    fn learn(&mut self, neurons: &[Neuron], output: &[Neuron]) {

    }
}
impl Display for Perceptron {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "It's perceptron")
    }
}
