use hyprland::data::Clients;
use hyprland::dispatch::Dispatch;
use hyprland::dispatch::DispatchType;
use hyprland::dispatch::WindowIdentifier;
use hyprland::prelude::*;
use hyprland::Result;
fn main() -> Result<()> {
    let clients = Clients::get()?.to_vec();
    // println!("{clients:#?}");
    for client in clients {
        println!("{}",client.address);
        if !client.title.contains("hyprclose") {
            Dispatch::call(DispatchType::CloseWindow(WindowIdentifier::Address(client.address)))?;
        }
    }
    Ok(())
}
