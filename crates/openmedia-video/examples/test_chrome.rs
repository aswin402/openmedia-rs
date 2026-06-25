use chromiumoxide::browser::{Browser, BrowserConfig};
use futures::StreamExt;

#[tokio::main]
async fn main() {
    println!("Building BrowserConfig...");
    let config = BrowserConfig::builder()
        .no_sandbox()
        .build()
        .unwrap();

    println!("Launching Browser...");
    let result = Browser::launch(config).await;
    match result {
        Ok((_browser, mut handler)) => {
            println!("SUCCESS: Browser launched!");
            tokio::spawn(async move {
                while let Some(h) = handler.next().await {
                    println!("Handler event: {:?}", h);
                }
            });
        }
        Err(err) => {
            println!("ERROR: Launch failed: {:?}", err);
        }
    }
}
