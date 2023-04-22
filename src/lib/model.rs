use dfdx::{nn::Linear, tensor, prelude::{Module, HasArrayData}};
use crate::encoding;


type ModelArch = (
  Linear<4, 64>,
  Linear<64, 64>,
  Linear<64, 2>,
);

#[derive(Clone, Debug)]
pub struct Model(ModelArch);

impl Model {
  // Initialize the weights and biases from 
  pub fn from_file(file: encoding::ModelFile) -> Self {
    let mut net: ModelArch =  Default::default();
    // Initialize first Linear layer with the
    // weights read from the capnp file
    net.0.weight = file.read_layer1_weights();
    net.0.bias = file.read_layer1_bias();
    // Initialize second Linear layer with the
    // weights read from the capnp file
    net.1.weight = file.read_layer2_weights();
    net.1.bias = file.read_layer2_bias();
    // Initialize third Linear layer with the
    // weights read from the capnp file
    net.2.weight = file.read_layer3_weights();
    net.2.bias = file.read_layer3_bias();
    Model(net)
  }

  pub fn predict(&self, state: [f32; 4]) -> u8 {
    let output = self.0.forward(tensor::tensor(state));
    let value = output.data();
    if value[0] > value[1] {
      return 0;
    }
    return 1;
  }

}