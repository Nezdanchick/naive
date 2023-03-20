#[derive(Copy, Clone)]
struct Neuron {
    value: f64
}
#[derive(Copy, Clone)]
struct Synapse {
    input: Neuron,
    weight: f64
}
impl Neuron {
    fn new(value: f64) -> Neuron{
        Neuron { value: value }
    }
    fn activate(&mut self, x: f64) {
        if self.value < 0.0 {
            self.value = 0.0;
            return;
        }
        self.value = 1.0 / (1.0 + self.value.powf(-x));
    }
    fn get(&self) -> f64 {
        self.value
    }
    fn add(&mut self, synapses:&[Synapse]) {
        for synapse in synapses {
            self.value += synapse.clone().calculate();
        }
        //self.activate(x);
    }
}
impl Synapse {
    fn calculate(&mut self) -> f64 {
        self.input.value * self.weight
    }
}
fn main() {
    let ne1 = Neuron::new(1.0);
    let ne2 = Neuron::new(0.0);
    let ne3 = Neuron::new(1.0);

    let syn1 = Synapse { input: ne1, weight: 0.4 };
    let syn2 = Synapse { input: ne2, weight: 0.7 };
    let syn3 = Synapse { input: ne3, weight: 0.5 };

    let mut out = Neuron::new(0.0);
    out.add(&[syn1, syn2, syn3]);

    print!("{}", out.get());
}

//
// [ne1] -> ->\
// [ne1] -> [out]
// [ne2] -> ->/
//