#![windows_subsystem = "windows"]

use trayicon::{MenuBuilder, TrayIcon, TrayIconBuilder};
use windows::Win32::System::Power::{SetThreadExecutionState, ES_CONTINUOUS, ES_SYSTEM_REQUIRED};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
};

#[derive(Clone, Eq, PartialEq, Debug)]
enum UserEvents {
    Exit,
    ToggleKeepAwake,
    RightClickTrayIcon,
}

struct MyApplication {
    tray_icon: TrayIcon<UserEvents>,
    keep_awake: bool,
}

fn main() {
    let event_loop = EventLoop::<UserEvents>::with_user_event().build().unwrap();
    let proxy = event_loop.create_proxy();

    let icon = include_bytes!("coffee.ico");

    let tray_icon = TrayIconBuilder::new()
        .sender(move |e: &UserEvents| {
            let _ = proxy.send_event(e.clone());
        })
        .icon_from_buffer(icon)
        .tooltip("Keep Awake")
        .on_right_click(UserEvents::RightClickTrayIcon)
        .menu(
            MenuBuilder::new()
                .checkable("Keep Awake", false, UserEvents::ToggleKeepAwake)
                .separator()
                .item("Exit", UserEvents::Exit),
        )
        .build()
        .unwrap();

    let mut app = MyApplication {
        tray_icon,
        keep_awake: false,
    };

    event_loop.run_app(&mut app).unwrap();
}

impl ApplicationHandler<UserEvents> for MyApplication {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: WindowEvent,
    ) {
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvents) {
        match event {
            UserEvents::Exit => event_loop.exit(),
            UserEvents::RightClickTrayIcon => {
                self.tray_icon.show_menu().unwrap();
            }
            UserEvents::ToggleKeepAwake => {
                if let Some(old_value) = self
                    .tray_icon
                    .get_menu_item_checkable(UserEvents::ToggleKeepAwake)
                {
                    let new_value = !old_value;
                    self.keep_awake = new_value;
                    let _ = self
                        .tray_icon
                        .set_menu_item_checkable(UserEvents::ToggleKeepAwake, new_value);

                    unsafe {
                        if new_value {
                            SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED);
                        } else {
                            SetThreadExecutionState(ES_CONTINUOUS);
                        }
                    }
                }
            }
        }
    }
}
