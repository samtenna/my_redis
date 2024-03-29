mod server;

fn main() {
    println!("Launching server");
    // TODO: handle this better than with an unwrap
    server::run_server("localhost", 8080).unwrap();
}
