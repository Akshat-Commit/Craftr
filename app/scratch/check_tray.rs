use tray_icon::TrayIconEvent;

fn main() {
    let _ = TrayIconEvent::receiver().try_recv();
}
