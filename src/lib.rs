mod neuron;
mod synapse;
pub mod networks;

pub use neuron::*;
pub use synapse::*;

#[test]
fn manual_percepthron() {
    let ne01 = Neuron::new(1.0);
    let ne02 = Neuron::new(0.0);
    let ne03 = Neuron::new(1.0);

    let syn01 = Synapse::new(ne01, 0.43);
    let syn02 = Synapse::new(ne02, 0.18);
    let syn03 = Synapse::new(ne03, -0.21);

    let syn04 = Synapse::new(ne01, 0.11);
    let syn05 = Synapse::new(ne02, 0.27);
    let syn06 = Synapse::new(ne03, 0.31);

    let mut ne11 = Neuron::empty();
    let mut ne12 = Neuron::empty();
    ne11.add(&[syn01, syn02, syn03]);
    ne12.add(&[syn04, syn05, syn06]);

    let syn11 = Synapse::new(ne11, 0.22);
    let syn12 = Synapse::new(ne12, 0.47);

    let mut out = Neuron::empty();
    out.add(&[syn11, syn12]);

    let output = out.get();
    {// prints info about neurons
        println!(
            "
-------------------------------------
[{na1:.2}] -> [{nb1:.5}] ->\\
[{na2:.2}] ->              [{out:.5}]
[{na3:.2}] -> [{nb2:.5}] ->/
-------------------------------------",
            na1 = ne01.get(),
            na2 = ne02.get(),
            na3 = ne03.get(),
            nb1 = ne11.get(),
            nb2 = ne12.get(),
            out = out.get()
        );

        println!("{}", output > 0.5);
    }
    if output <= 0.60005 && output >= 0.60006 {
        panic!()
    }
}