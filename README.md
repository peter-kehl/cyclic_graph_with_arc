Demonstrating that `Arc` on its own doesn't allow

As per https://doc.rust-lang.org/std/sync/struct.Arc.html: "Shared references in Rust disallow mutation by default, and Arc is no exception: you cannot generally obtain a mutable reference to something inside an Arc."