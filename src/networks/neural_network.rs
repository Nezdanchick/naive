use crate::neuron::Neuron;

pub trait NeuralNetwork {
    fn input(&mut self, neurons: &[Neuron]);
    fn learn(&mut self, neurons: &[Neuron], output: &[Neuron]);
}