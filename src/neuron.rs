use crate::synapse::Synapse;
use std::f64::consts::E as euler;

#[derive(Copy, Clone)]
pub struct Neuron {
    _value: f64,
}

impl Neuron {
    pub fn new(value: f64) -> Neuron {
        Neuron { _value: value }
    }
    pub fn empty() -> Neuron {
        Neuron { _value: 0.0 }
    }
    fn activate(&mut self) {
        self._value = 1.0 / (1.0 + euler.powf(-self._value));
    }
    pub fn get(&self) -> f64 {
        self._value
    }
    pub fn add(&mut self, synapses: &[Synapse]) {
        for synapse in synapses {
            self._value += synapse.clone().calculate();
        }
        self.activate();
    }
}
