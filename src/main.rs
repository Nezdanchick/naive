use naive::neuron::*;
use naive::synapse::*;

//
// [ne01] -> [ne11] ->\
// [ne02]           [out]
// [ne03] -> [ne12] ->/
//

fn main() {
    let ne01 = Neuron::new(1.0);
    let ne02 = Neuron::new(0.0);
    let ne03 = Neuron::new(1.0);

    let syn01 = Synapse::new(ne01, 0.43);
    let syn02 = Synapse::new(ne02, 0.18);
    let syn03 = Synapse::new(ne03, -0.21);

    let syn04 = Synapse::new(ne01, 0.11);
    let syn05 = Synapse::new(ne02, 0.27);
    let syn06 = Synapse::new(ne03, 0.31);

    let mut ne11 = Neuron::new(0.0);
    let mut ne12 = Neuron::new(0.0);
    ne11.add(&[syn01, syn02, syn03]);
    ne12.add(&[syn04, syn05, syn06]);

    let syn11 = Synapse::new(ne11, 0.22);
    let syn12 = Synapse::new(ne12, 0.47);

    let mut out = Neuron::new(0.0);
    out.add(&[syn11, syn12]);

    print!("{}", out.get());
}