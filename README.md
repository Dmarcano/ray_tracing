# Ray Tracing Using Rust. 

This is a simple ray tracer built using Rust. This is more or less a very faithfull recreation of [the ray tracing in a weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
 online tutorial and functions as a fun way to experiment a bit more with Rust's features.

### Feature List

- [x] Vector class over generic types
- [x] Rendering simple gradients to a file
- [x] Rendering to a realtime window ([winit](https://github.com/rust-windowing/winit) and [pixels](https://github.com/parasyte/pixels))
- [ ] Rendering spheres (WIP)
- [ ] Arbitrary object rendering
- [ ] Rendering diffuse materials
- [ ] Parallelize the computation ([rayon](https://github.com/rayon-rs/rayon)) 
- [ ] Moveable Camera
- [ ] WebAssembly

### Example

The most current image output by my raytracer should be out below. The raytracer generates a ppm image and I use ffmpeg to convert the image to a png.

![most current output](public/out.png)

### building 

To build and run one can simply use cargo

```
cargo build
```

```
cargo run
```

most dependencies should resolve themselves