# Rust Concurrency & Memory Safety Reference

This document explains the technical "Why" behind the Tire Pressure Monitor implementation.

## 1. The Borrow Checker's Core Rule
Rust enforces memory safety through a simple rule at compile time:
> **Aliasing XOR Mutation:** You can have many immutable references (`&T`) OR exactly one mutable reference (`&mut T`), but never both at the same time.

In our CLI, this means the **Receiver** cannot read the pressure while the **Sender** is still writing to that same memory object.

## 2. Move Semantics in Closures
When we use `thread::spawn(move || { ... })`:
- **The `move` keyword:** Forces the closure to take **Ownership** of any captured variables.
- **The Lifecycle:** Once a variable is moved into the thread, it is dropped when the thread finishes.
- **The "Copy" Exception:** Types like `f32` (pressure) or `u32` implement the `Copy` trait. They are duplicated rather than moved.

## 3. Channels (MPSC)
`std::sync::mpsc` stands for **Multi-Producer, Single-Consumer**.

| Component | Role | Ownership Behavior |
| :--- | :--- | :--- |
| **Sender (`tx`)** | The Virtual Sensor | Can be `.clone()`-ed to create multiple senders. |
| **Receiver (`rx`)** | The CLI Display | Only one exists. It "owns" the data once it pulls it out. |
| **Data Packet** | `tireData` struct | Ownership is **moved** into the channel, then **moved** to the receiver. |



## 4. Why `&sensors` is required in a Loop
Even if a `Vec` contains `Copy` types (like numbers), the `Vec` itself is **allocated on the Heap** and is **not Copy**.

```rust
loop {
    for s in &sensors { // BORROWING: The vector stays put.
        // ...
    } // The borrow ends here, sensors is still available for the next loop.
}
