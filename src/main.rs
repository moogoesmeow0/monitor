use notify::{EventKind, RecursiveMode, Watcher};
use std::error::{self, Error};
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread::AccessError;
use tokio::net::TcpListener;
use tokio::time::sleep;

mod math;
mod plot;
mod server;
mod shared;
mod util; // Add this

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create shared state
    let shared_state = shared::new_shared_state();

    // Initialize with current data
    if let Ok(initial_data) = util::read() {
        if let Ok(mut guard) = shared_state.write() {
            guard.update_points(initial_data);
        }
    }

    if let Ok(points) = shared_state.read() {
        println!("{}", points.points.len());
    }

    let file_watcher = watch(shared_state.clone());
    let rocket_server = async {
        if let Err(e) = server::rocket(shared_state.clone()).launch().await {
            eprintln!("Rocket server error: {}", e);
        }
    };

    tokio::select! {
        _ = file_watcher => println!("File watcher stopped"),
        _ = rocket_server => println!("Rocket server stopped"),
    };

    Ok(())
}

async fn watch(shared_state: shared::SharedState) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(new_data) = util::read() {
        if let Ok(mut guard) = shared_state.write() {
            guard.update_points(new_data);
        } else {
            return Err(Box::new(todo!()));
        }
    }

    let _ = &plot::plot()?;

    let (std_tx, std_rx) = std::sync::mpsc::channel();

    let mut watcher = notify::recommended_watcher(move |res| {
        let _ = std_tx.send(res);
    })?;

    watcher
        .watch(Path::new("./data.csv"), RecursiveMode::Recursive)
        .unwrap();

    // Bridge the std channel to tokio in a separate task
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    tokio::spawn(async move {
        for res in std_rx {
            if tx.send(res).await.is_err() {
                break;
            }
        }
    });

    while let Some(res) = rx.recv().await {
        match res {
            Ok(event) => {
                if event.kind.is_modify() || event.kind.is_create() {
                    println!("File changed: {:?}", event.paths);

                    if let Ok(new_data) = util::read() {
                        shared_state.write().unwrap().update_points(new_data);
                        println!(
                            "Updated shared state with {} points",
                            shared_state.read().unwrap().points.len()
                        );
                    }

                    let _ = &plot::plot()?;
                } else if event.kind.is_remove() {
                    println!("File removed: {:?}", event.paths);
                    return Err(Box::new(util::Error::FileRemoved));
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
    Ok(())
}
