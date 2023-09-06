# Wrap-Up

> This is the end of the tutorial. I hope you enjoyed it and learned something!

What have we learned today?

**System Threads**

* Make system threads, independent or scoped.
* Use `rayon` to make parallel iterators.
* How Rust saves you from data races
* Use atomics, mutexes, read-write locks to share data between threads.
* Use `Arc` to reference count data when ownership is unclear.
* Use Interior Mutability to clean up and speed-up concurrent access to shared data.
* Use channels to send data between threads.

**Async**

* The basics of Tokio and async/await.
* Join, yield, sleep, and select.
* Blocking and async.
* Async channels.
* Connecting the Async and Sync worlds.
* A complete TCP server and client.