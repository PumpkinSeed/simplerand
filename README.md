# simplerand

Simple and fast random number generator

- [Motivation](#motivation)
- [Usage](#usage)

### Motivation

I've worked on a fake data generator, which strongly relies on random number generation. The package, which serves this purpose in the world of Rust was the [rand](https://crates.io/crates/rand) package. That's a cool package, if you want to have enterprise-level random generation. But for a simple fake data generator it's huge. Whatsoever, it takes 400+ microsec to generate a random number. ([struggle](#rand-package))

So I`ve created a much simpler pseudo-random generator based on a very simple seeding mechanism. That won't serve cryptographic random generations! So it's only useful for projects like a fake data generator where a simple and fast solution is needed. Thid way I can generate random numbers in **~200 nanosec**.

The random number generation is strongly based on the **linear congruential generators**, so if you are familiar with the algorithm you can imagine how simple it is. 

### Usage

```rust
extern crate simplerand;

use simplerand::{randn, rand_range};

fn main() {
    let data = randn(10000);
    println!("data: {}", data); // output: something between 0 and 10000

    let data = rand_range(3000, 10000);
    println!("data: {}", data); // output: something between 3000 and 10000
}
```

##### Singleton implementation

```rust
#[macro_use]
extern crate lazy_static;
extern crate simplerand;

use std::sync::Mutex;

lazy_static! {
    static ref rand: Mutex<simplerand::Rng> = Mutex::new(simplerand::Rng::new());
}

fn main() {
    let number = RNG.lock().unwrap().rand_range(0, 10000; // output: something between 0 and 10000
}
```

#### rand package

Since it took 400 microsec to generate a random number I thought it's because of the seeding mechanism. I wanted to setup the rand's ThreadRng as a static singleton variable on top of my declaration. I also used lazy_static to achive it, but it's not designed for that purpose, so I couldn't make it happen. (If someone has a solution for that I'm curious to see.)