# Rust Ray Marching

An implementation of the [spherical ray marching](https://typhomnt.github.io/teaching/ray_tracing/raymarching_intro/#marching) algorithm in rust:

```rust
fn march<S: SdfInfo<N>>(
  &self,
  sdf: &S,
  Ray { mut pos, dir }: Ray<N>,
) -> Result<S::Info, MarchError> {
  for _ in 0..self.max_iter {
    let dist = sdf.call(pos);
    if dist > self.miss_error {
      return Err(MarchError::Diverges(dist));
    }
    if dist < self.hit_error {
      return Ok(sdf.info(pos));
    }
    pos = pos + dist * dir;
  }

  Err(MarchError::MaxIter(self.max_iter))
}
```

I know this should be done using webgpu but:

1. I'd have to do transpilation :(
2. I'm going to try to optimise performance a fair bit here

## Examples

There's 2 primary examples:

### `perf.rs`

This example tests the raw performance of SDF rendering\
and is intended for performance optimisation and can be run with:

```bash
cargo flamegraph --example perf
```

[cargo-flamegraph](https://crates.io/crates/flamegraph)

### Yew app

This example provides a slightly nicer way of displaying\
renders for SDF objects, via a Yew web app.\
This can be run with:

```bash
trunk serve ./yew/index.html --open
```

[trunk](https://crates.io/crates/trunk)
