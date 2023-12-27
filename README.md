This Project was an attempt to create something that is able to abstract away data flow in an application with multiple components.
Each Component Acts as a (stateful) function that just processes data, and Connections are made from the outside. In the End I've noticed that this is not what I've wanted and I moved on.
Spoiler: Rust `Iterator`s and `FnMut`s are the things I really wanted. Though at the time I didn't know better.
