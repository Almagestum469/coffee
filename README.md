# Coffee - Keep Your PC Awake

A lightweight system tray application that prevents your Windows PC from going to sleep.

## Features

- Simple system tray interface
- Toggle keep-awake mode with a single click
- Minimal resource usage
- Runs in the background

## Usage

1. Run the application
2. Look for the coffee cup icon in your system tray
3. Right-click the icon to show the menu
4. Click "Keep Awake" to toggle the keep-awake mode
   - When checked, your PC will not go to sleep
   - When unchecked, normal sleep settings apply
5. Select "Exit" to close the application

## Building from Source

### Prerequisites

- Rust toolchain
- Windows development environment

### Build Steps

```bash
git clone <repository-url>
cd coffee
cargo build --release
```

The compiled executable will be located in `target/release/coffee.exe`

## Technical Details

Built with:

- Rust
- winit - for event handling
- trayicon - for system tray integration
- windows-rs - for Windows API integration

## License

MIT License
