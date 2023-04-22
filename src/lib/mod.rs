pub mod encoding;
pub mod model;
pub mod server;
pub mod model_capnp;

#[cfg(test)]
mod tests {
    use crate::{encoding, model::Model};

    #[test]
    fn known_predictions() {
      // Initialize the model from the model.o file
      let model_file = encoding::ModelFile::from("model.o");
      let model = Model::from_file(model_file);
      
      // Test against known predictions, based on the python implementation 
      // https://huggingface.co/sb3/ppo-CartPole-v1
      assert_eq!(model.predict([-0.04456399, 0.04653909 ,0.01326909,-0.02099827]), 0);
      assert_eq!(model.predict([-0.04568531,-0.14921477 ,0.01811073 ,0.28565928]), 1);
      assert_eq!(model.predict([-0.04866961, 0.04564426 ,0.02382392, -0.00125707]), 1);
      assert_eq!(model.predict([-0.04775672, 0.24041659 ,0.02379878, -0.2863291 ]), 0);
    }
}