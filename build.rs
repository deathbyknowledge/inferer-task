use pyo3::prelude::*;

fn main() {
    /*
    **********************************************
    Our build process is extra-customised. Twice!
    **********************************************
    1. First, we want to make sure the policy.pth file
       has been decoded and encoded again into a format
       we can easily interact from Rust. I chose Capn' Proto! (capnp)
       Since the original file is encoded with `cloudpickle` and
       there is no easy way to use it direclty from Rust, we'll use
       the Python interface instead. From there, use torch to open the 
       model file and then store the weights and biases in our custom
       capnp format.

    2. Rust's Capnp library makes use of build scripts (exactly what this
       file is) to auto-generate the code responsible for using the Capnp
       types. Python doesn't need this step, it generates all the necessary
       accessor functions for a given schema at runtime.
    */


    // Tell cargo to re-run this build script when model.o or policy.pth change
    println!("cargo:rerun-if-changed=model.o");
    println!("cargo:rerun-if-changed=model/policy.pth");

    // Setup the python interpreter
    pyo3::prepare_freethreaded_python();

    // Read policy.pth with torch and encode the model
    // with the Capnp schema
    Python::with_gil(|py| {
        let fun: Py<PyAny> = PyModule::from_code(
            py,
            "def build_function(*args, **kwargs):
                import torch
                import capnp

                capnp.remove_import_hook()
                model_capnp = capnp.load('cereal/model.capnp')

                model = torch.load('model/policy.pth', weights_only=True)
                l1_w, l1_b = (model['mlp_extractor.policy_net.0.weight'], model['mlp_extractor.policy_net.0.bias'])
                l2_w, l2_b = (model['mlp_extractor.policy_net.2.weight'], model['mlp_extractor.policy_net.2.bias'])
                l3_w, l3_b = (model['action_net.weight'], model['action_net.bias'])

                m = model_capnp.Model.new_message()
                layer1 = m.layer1
                layer1.weights = l1_w.tolist()
                layer1.bias = l1_b.tolist()
                layer2 = m.layer2
                layer2.weights = l2_w.tolist()
                layer2.bias = l2_b.tolist()
                layer3 = m.layer3
                layer3.weights = l3_w.tolist()
                layer3.bias = l3_b.tolist()
                m.write(open('model.o', 'wb'))",
            "",
            "",
        ).expect("wtf somehow broke")
        .getattr("build_function").expect("wtf somehow broke 2")
        .into();

        // call object without any arguments
        fun.call0(py).unwrap()
    });


    // 2. Compile model.capnp to generate Rust accessor code
    // for the schema
    capnpc::CompilerCommand::new()
      .src_prefix("cereal")
      .file("cereal/model.capnp")
//      .default_parent_module(vec!["encoding".to_string()])
      .run().expect("schema compiler command");
}
  