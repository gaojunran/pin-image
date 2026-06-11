# pin-image

A lightweight tool to pin images to your desktop corners.

Borderless, always-on-top, transparent window — images float like sticky notes on your desktop.

## Install

```bash
mise use -g github:gaojunran/pin-image
```

Or build from source:

```bash
git clone https://github.com/gaojunran/pin-image.git
cd pin-image
cargo build --release
```

## Usage

```bash
pin-image <IMAGE_PATH> [OPTIONS]
```

### Options

| Option | Default | Description |
|--------|---------|-------------|
| `<IMAGE_PATH>` | required | Image file path (mutually exclusive with `--clipboard`) |
| `--clipboard` | `false` | Read image from clipboard |
| `--width <WIDTH>` | original image width | Window width |
| `--height <HEIGHT>` | original image height | Window height |
| `--location <LOCATION>` | `top-right` | Window position: `top-left`, `top-center`, `top-right`, `bottom-left`, `bottom-right` |
| `--left-click <ACTION>` | `nothing` | Left click action: `copy`, `copy-close`, `close`, `nothing` |
| `--right-click <ACTION>` | `copy` | Right click action: `copy`, `copy-close`, `close`, `nothing` |
| `--double-click <ACTION>` | `close` | Double click action: `copy`, `copy-close`, `close`, `nothing` |

### Click Actions

| Value | Description |
|-------|-------------|
| `copy` | Copy image to clipboard |
| `copy-close` | Copy image to clipboard, then close window |
| `close` | Close window |
| `nothing` | No action |

### Examples

Pin an image to the top-right corner (default):

```bash
pin-image screenshot.png
```

Pin to top-left, left click copies, right click closes:

```bash
pin-image screenshot.png --location top-left --left-click copy --right-click close
```

Pin an image from clipboard:

```bash
pin-image --clipboard
```

Specify window size:

```bash
pin-image photo.jpg --width 300 --height 200
```

## License

MIT
