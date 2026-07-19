#Topics To Learn

##Concurrenecy

Rust has a powerfull Concurrenecy feature which unlike other languages because it provides safety
Rust compiler is very usefull for this as it picks out the errors in compile time rather than in
run time

```rust
std::thread

```

1.Rust uses a messaging sytem to pass values around a thread it is called a chanel (like a river)
it has a transmitter(start of the river) and a receiver (end of the river) and if either one of them
stops the channel stops as well.
We can declare a chanel using the mpsc::chanel in which mspc stands for multiple source single consumer
Which basically tells that there can be more than one transmitter(sender) but there has to only one reciever

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}
```

2.Rust
