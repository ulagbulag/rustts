use tch::Tensor;

pub fn normalize(input: &Tensor) -> Tensor {
    let p = 2;
    let dim = &[1];
    let eps = 1e-12;

    let denom = input
        .norm_scalaropt_dim(p, dim, true)
        .clamp_min(eps)
        .expand_as(input);
    input / denom
}
