use crate::neuron::Neuron;

#[derive(Copy, Clone)]
pub struct Synapse {
    neuron: Neuron,
    weight: f64
}
impl Synapse {
    pub fn new(input: Neuron, weight: f64) -> Synapse{
        Synapse { neuron: input, weight: weight }
    }
    pub fn calculate(&self) -> f64 {
        self.neuron.get() * self.weight
    }
}