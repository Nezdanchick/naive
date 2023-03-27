use crate::neuron::Neuron;

#[derive(Copy, Clone)]
pub struct Synapse {
    _neuron: Neuron,
    _weight: f64
}
impl Synapse {
    pub fn empty() -> Synapse{
        Synapse { _neuron: Neuron::empty(), _weight: 0.0 }
    }
    pub fn new(input: Neuron, _weight: f64) -> Synapse{
        Synapse { _neuron: input, _weight: _weight }
    }
    pub fn calculate(&self) -> f64 {
        self._neuron.get() * self._weight
    }
}