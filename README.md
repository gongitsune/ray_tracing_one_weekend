# Ray tracing in one weekend, in Rust

This is an implementation in Rust of Peter Shirley's "Ray Tracing In One Weekend" book.
This is a Rust implementation of Peter Shirley's book [Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html). I used [ray-tracing-in-one-weekend](ray-tracing-in-one-weekend) as a reference for the implementation. It was very helpful and a great implementation, especially since I had no knowledge of parallelization using `rayon`. I would like to express my gratitude here.

Instead of implementing my own `vec3`, I used [`nalgebra`](https://github.com/dimforge/nalgebra).
For random numbers I used [`rand`](https://github.com/rust-random/rand).
For parallelization I used [`rayon`](https://github.com/rayon-rs/rayon).
I used [`indicatif`](https://github.com/console-rs/indicatif) to check progress. It also supports `rayon` and I was able to display a progress bar very easily.

## How to use

First, download the latest executable from Release and place `raytra` or `raytra.exe` in any directory.

- Simple run

  Bash

  ```bash
  ./raytra
  ```

  PowerShell

  ```ps
  .\raytra.exe
  ```

- Helps

  Bash

  ```bash
  ./raytra --help
  ```

  PowerShell

  ```ps
  .\raytra.exe --help
  ```

- Options -> ImageWidth: 512px, SamplesPerPixel: 50, MaxDeps: 100

  Bash

  ```bash
  ./raytra -w 512 -s 50 -d 100
  ```

  PowerShell

  ```ps
  .\raytra.exe -w 512 -s 50 -d 100
  ```

![Ray Tracing](https://raytracing.github.io/images/img-1.21-book1-final.jpg)
