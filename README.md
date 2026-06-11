# pin-image

将图片钉在桌面角落的轻量工具。

无边框、置顶、透明窗口，让图片像便签一样浮在桌面上。

## 安装

```bash
mise use -g github:gaojunran/pin-image
```

或从源码构建：

```bash
git clone https://github.com/gaojunran/pin-image.git
cd pin-image
cargo build --release
```

## 用法

```bash
pin-image <IMAGE_PATH> [OPTIONS]
```

### 参数

| 参数 | 默认值 | 说明 |
|------|--------|------|
| `<IMAGE_PATH>` | 必选 | 图片文件路径（与 `--clipboard` 二选一） |
| `--clipboard` | `false` | 从剪贴板读取图片 |
| `--width <WIDTH>` | 图片原始宽度 | 窗口宽度 |
| `--height <HEIGHT>` | 图片原始高度 | 窗口高度 |
| `--location <LOCATION>` | `top-right` | 窗口位置：`top-left`、`top-center`、`top-right`、`bottom-left`、`bottom-right` |
| `--left-click <ACTION>` | `nothing` | 左键行为：`copy`、`copy-close`、`close`、`nothing` |
| `--right-click <ACTION>` | `copy` | 右键行为：`copy`、`copy-close`、`close`、`nothing` |
| `--double-click <ACTION>` | `close` | 双击行为：`copy`、`copy-close`、`close`、`nothing` |

### 点击行为说明

| 值 | 说明 |
|----|------|
| `copy` | 复制图片到剪贴板 |
| `copy-close` | 复制图片到剪贴板后关闭窗口 |
| `close` | 关闭窗口 |
| `nothing` | 无操作 |

### 示例

钉住图片到右上角（默认）：

```bash
pin-image screenshot.png
```

钉住图片到左上角，左键复制，右键关闭：

```bash
pin-image screenshot.png --location top-left --left-click copy --right-click close
```

从剪贴板读取图片并钉住：

```bash
pin-image --clipboard
```

指定窗口大小：

```bash
pin-image photo.jpg --width 300 --height 200
```

## 许可证

MIT
