use std::f64::consts::E;
use crate::synapse::Synapse;

#[derive(Copy, Clone)]
pub struct Neuron {
    value: f64
}

impl Neuron {
    pub fn new(value: f64) -> Neuron{
        Neuron { value: value }
    }
    fn activate(&mut self) {
        self.value = 1.0 / (1.0 + E.powf(-self.value));
    }
    pub fn get(&self) -> f64 {
        self.value
    }
    pub fn add(&mut self, synapses:&[Synapse]) {
        for synapse in synapses {
            self.value += synapse.clone().calculate();
        }
        self.activate();
    }
}