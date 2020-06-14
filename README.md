# srand
Simple and fast random number generator

- [Usage](#usage)
- [Motivation](#motivation)

### Usage

```
fn main() {
    let mut rng = Rng::new();

    let data = rng.randn(10000);
    println!("data: {}", data); // output: something between 0 and 10000
}
```

##### Singleton implementation

```
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

### Motivation