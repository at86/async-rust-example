// use std::intrinsics::rustc_peek;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();

    task("task1", now.clone())?;
    task("task2", now.clone())?;
    task("task3", now.clone())?;
    Ok(())
}

fn task(label: &str, now: std::time::Instant) -> Result<(), Box<dyn std::error::Error>> {
    // Simulate network delay using thread sleep for 2 seconds
    println!(
        "OS Thread {:?} - {} started: {:?}",
        std::thread::current().id(),
        label,
        now.elapsed(),
    );
    sen::atlog!("simulate network delay, 1s");
    sleep(std::time::Duration::from_secs(1));

    // Write to server - server will echo this back to us with 8 second delay
    let mut stream = TcpStream::connect("127.0.0.1:6142")?;
    sen::atlog!("begin stream::write_all()..", 1,);
    sen::atlog!();
    stream.write_all(label.as_bytes())?;
    println!(
        "OS Thread {:?} - {} written: {:?}",
        std::thread::current().id(),
        label,
        now.elapsed(),
    );

    // Read 5 chars we expect (to avoid dealing with EOF, etc.)
    let mut buffer = [0; 5];
    sen::atlog!("begin stream::read_exact() `svr doing and response` 2s");
    stream.read_exact(&mut buffer)?;
    sen::atlog!("begin stream::shutdown()", std::thread::current().id());
    stream.shutdown(std::net::Shutdown::Both)?;
    println!(
        "OS Thread {:?} - {} read: {:?}",
        std::thread::current().id(),
        label,
        now.elapsed(),
    );

    sen::atlog!("begin simulate computation, 1s");
    // Simulate computation work by sleeping actual thread for 4 seconds
    sleep(std::time::Duration::from_secs(1));
    println!(
        "OS Thread {:?} - {} finished: {:?}",
        std::thread::current().id(),
        std::str::from_utf8(&buffer)?,
        now.elapsed(),
    );
    Ok(())
}
