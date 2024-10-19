use hyprland::async_closure;
use hyprland::dispatch;
use hyprland::dispatch::Dispatch;
use hyprland::dispatch::DispatchType;
use hyprland::dispatch::Position;
use hyprland::dispatch::WindowIdentifier;
use hyprland::event_listener::AsyncEventListener;

// Constants for window resizing in 16:9 aspect ratio
const DEFAULT_WIDTH: i16 = 700;
const DEFAULT_HEIGHT: i16 = 394;

// TODO: Look into using corner positioning instead of exact positioning
// Constants for window positioning
const DEFAULT_X: i16 = 2727;
const DEFAULT_Y: i16 = 57;

#[tokio::main(flavor = "current_thread")]
async fn main() -> hyprland::Result<()> {
    let mut listener = AsyncEventListener::new();

    listener.add_window_open_handler(async_closure! {|window|
        if window.window_title == "Picture-in-Picture" {
            let identifier = WindowIdentifier::Address(window.window_address.clone());

            // Handle ToggleFloating
            if let Err(e) = dispatch!(async; ToggleFloating, None).await {
                println!("Error toggling floating: {}", e);
            }

            // Handle ResizeWindowPixel
            if let Err(e) = dispatch!(
                async;
                ResizeWindowPixel,
                Position::Exact(DEFAULT_WIDTH, DEFAULT_HEIGHT),
                identifier.clone()
            ).await {
                println!("Error resizing window: {}", e);
            }

            // Handle MoveWindowPixel
            if let Err(e) = dispatch!(
                async;
                MoveWindowPixel,
                Position::Exact(DEFAULT_X, DEFAULT_Y),
                identifier.clone()
            ).await {
                println!("Error moving window: {}", e);
            }
        }
    });

    listener.start_listener_async().await?;

    Ok(())
}
