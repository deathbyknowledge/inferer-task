@0xdbb9ad1f14bc0b36;
# There has to be a better/optimal cross-platform way of encoding this
struct Model {
    layer1 @0 :Layer;
    layer2 @1 :Layer;
    layer3 @2 :Layer;
}

struct Layer {
    weights @0 :List(List(Float32));
    bias @1 :List(Float32);
}