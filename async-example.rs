// We'll need the async-std crate to provide async read/write operations
use async_std::fs::File;
use async_std::prelude::*;

// This is an async function. It returns a Future that will be completed
// once the file has been written.
async fn write_to_file() -> std::io::Result<()> {
    let mut file = File::create("test.txt").await?;
    file.write_all(b"Hello, async world!").await?;

    Ok(())
}

fn main() {
    // The async-std runtime allows us to run async functions from synchronous code.
    // It sets up a thread pool and runs tasks on it.
    async_std::task::block_on(write_to_file());
}
