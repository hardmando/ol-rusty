use engine::Engine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting ol-rusty Engine...");

    let engine = Engine::new();
    match engine.run() {
        Ok(()) => {
            println!("Engine shut down successfully.");
            Ok(())
        }
        Err(e) => {
            eprintln!("Engine error: {}", e);
            Err(e)
        }
    }
}

