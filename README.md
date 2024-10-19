# HyprPiPManager

**HyprPiPManager** is a Rust-based tool designed to enhance the Hyprland compositor by managing Picture-in-Picture (PiP) windows more effectively. By default, Hyprland treats PiP windows as regular tiled windows, which can disrupt the intended layout and user experience. HyprPiPManager addresses this limitation by automatically setting PiP windows to floating mode and providing users with the ability to control their resizing and positioning seamlessly.

## 🛠️ Features

- **Automatic Floating:** Detects PiP windows and sets them to floating mode to prevent unwanted tiling.
- **Custom Resizing:** Allows users to define and apply custom sizes to PiP windows.
- **Custom Positioning:** Enables users to specify exact screen coordinates for PiP windows, ensuring consistent and optimal placement.
- **Real-time Monitoring:** Listens for window events to manage PiP windows dynamically.
- **Easy Integration:** Works seamlessly with Hyprland.

## 🚀 Installation

### Prerequisites

- **Rust and Cargo:** Ensure you have Rust and Cargo installed. You can install them using [rustup](https://rustup.rs/).

### Build

**Clone the Repository:**

```bash
git clone https://github.com/PaytonWebber/hypr-pip-manager.git
cd hypr-pip-manager
```

**Build the Project:**

```bash
cargo build --release
```

### Run
```bash
./target/release/hypr-pip-manager
```

### Hyprland Usage

Add the following line to your Hyprland config file, replacing `{path-to-binary}` with the actual path to the `hypr-pip-manager` binary:

```bash
exec-once = {path-to-binary}/hypr-pip-manager
```

## 🔧 Configuration

Currently, the values for resizing and positioning are hardcoded as constants at the top of `src/main.rs`:

```rust
// Constants for window resizing in 16:9 aspect ratio
const DEFAULT_WIDTH: i16 = 700;
const DEFAULT_HEIGHT: i16 = 394;

// Constants for window positioning
const DEFAULT_X: i16 = 2727;
const DEFAULT_Y: i16 = 57;
```

You can modify these values to suit your setup.

## 🔮 Future Plans

- **Workspace and Monitor Specification:** Add functionality to specify which workspaces and monitors PiP windows should be moved to.
- **Enhanced Configuration Options:** Provide an easier way for users to customize their configurations, such as through a configuration file or command-line options.
