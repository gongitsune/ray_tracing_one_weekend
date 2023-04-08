# Ray tracing in one weekend, in Rust

This is an implementation in Rust of Peter Shirley's "Ray Tracing In One Weekend" book.
This is a Rust implementation of Peter Shirley's book [Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html). I used [ray-tracing-in-one-weekend](ray-tracing-in-one-weekend) as a reference for the implementation. It was very helpful and a great implementation, especially since I had no knowledge of parallelization using rayon. I would like to express my gratitude here.

Instead of implementing my own `vec3`, I used [`nalgebra`](https://github.com/dimforge/nalgebra).
For random numbers I used [`rand`](https://github.com/rust-random/rand).
For parallelization I used [`rayon`](https://github.com/rayon-rs/rayon).
I used [`indicatif`](https://github.com/console-rs/indicatif) to check progress. It also supports `rayon` and I was able to display a progress bar very easily.

![Ray Tracing](https://raytracing.github.io/images/img-1.21-book1-final.jpg)
