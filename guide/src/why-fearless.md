# Why be Fearless?

Rust's big promises are:

* Memory safety
* High Performance
* Happy Programmers (see the "Most Beloved Language" award!)
* Fearless Concurrency

> So what does "Fearless Concurrency" mean?

*Concurrency* is the ability to do more than one thing at a time. It's often divided into two categories:

* **Threaded Concurrency** - Using multiple threads of execution to do more than one thing at a time.
* **Async Concurrency** - Maintain a work queue that "wakes up" when work is available. Tasks spend a lot of time waiting for databases, REST calls, etc. to return. Async concurrency can run on a single thread, but it can also run on multiple threads.

In many languages, concurrency is **hard**:

* Ownership and destruction of data can be tricky to figure out. Are all threads done with an item? Can we destroy it?
* Pointers and references are problematic, because they can be invalidated by other threads.
* Sharing data between threads can be dangerous. You have to be absolutely positive that you've protected the data from being modified by multiple threads at the same time.

Rust protects you from all of these:

* The **ownership system** ensures that you know who owns some data, or that you can explicitly opt in to a shared ownership system---with minimal overhead.
* The **borrow checker** prevents you from creating dangling pointers.
* The **borrow checker** prevents concurrent access to data, unless it is explicitly marked as safe to do so.

Rust protects you from these problems---at compile time. You can write high-performance code without fearing these common issues.