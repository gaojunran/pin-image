use std::path::PathBuf;

use arboard::Clipboard;
use clap::Parser;
use eframe::App;
use image::GenericImageView;

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum Location {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum ClickAction {
    Copy,
    CopyClose,
    Close,
    Nothing,
}

#[derive(Parser)]
#[command(name = "pin-image", version, about = "将图片钉在桌面角落")]
struct Args {
    /// 要钉住的图片路径
    image_path: Option<PathBuf>,

    /// 从剪贴板读取图片，而不是从文件读取
    #[arg(long)]
    clipboard: bool,

    /// 窗口宽度
    #[arg(long)]
    width: Option<u32>,

    /// 窗口高度
    #[arg(long)]
    height: Option<u32>,

    /// 桌面位置
    #[arg(long, value_enum, default_value = "top-right")]
    location: Location,

    /// 左键行为
    #[arg(long, value_enum, default_value = "nothing")]
    left_click: ClickAction,

    /// 右键行为
    #[arg(long, value_enum, default_value = "copy")]
    right_click: ClickAction,

    /// 双击行为
    #[arg(long, value_enum, default_value = "close")]
    double_click: ClickAction,
}

struct PinImage {
    texture: Option<egui::TextureHandle>,
    img_size: [u32; 2],
    location: Location,
    left_click: ClickAction,
    right_click: ClickAction,
    double_click: ClickAction,
    positioned: bool,
    rgba_for_copy: Vec<u8>,
}

impl App for PinImage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.positioned {
            self.position_window(ctx);
            self.positioned = true;
        }

        let right_clicked = ctx.input(|i| i.pointer.secondary_clicked());

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = &self.texture {
                let size = texture.size_vec2();
                let response = ui.image((texture.id(), size)).interact(egui::Sense::click_and_drag());

                if response.double_clicked() {
                    self.handle_click(ctx, self.double_click);
                } else if response.clicked() {
                    self.handle_click(ctx, self.left_click);
                }
                if right_clicked {
                    self.handle_click(ctx, self.right_click);
                }

                if response.dragged() {
                    let delta = response.drag_delta();
                    let current_pos = ctx.screen_rect().min;
                    ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(
                        current_pos + delta,
                    ));
                }
            }
        });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0; 4]
    }
}

impl PinImage {
    fn new(
        ctx: &egui::Context,
        rgba: Vec<u8>,
        img_size: [u32; 2],
        location: Location,
        left_click: ClickAction,
        right_click: ClickAction,
        double_click: ClickAction,
    ) -> Self {
        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            [img_size[0] as usize, img_size[1] as usize],
            &rgba,
        );
        let texture = ctx.load_texture("pinned-image", color_image, Default::default());

        Self {
            texture: Some(texture),
            img_size,
            location,
            left_click,
            right_click,
            double_click,
            positioned: false,
            rgba_for_copy: rgba,
        }
    }

    fn position_window(&self, ctx: &egui::Context) {
        let screen = ctx.input(|i| i.screen_rect);
        let screen_size = screen.size();
        let win_w = self.img_size[0] as f32;
        let win_h = self.img_size[1] as f32;

        let pos = match self.location {
            Location::TopLeft => egui::Pos2::new(0.0, 0.0),
            Location::TopCenter => egui::Pos2::new((screen_size.x - win_w) / 2.0, 0.0),
            Location::TopRight => egui::Pos2::new(screen_size.x - win_w, 0.0),
            Location::BottomLeft => egui::Pos2::new(0.0, screen_size.y - win_h),
            Location::BottomRight => {
                egui::Pos2::new(screen_size.x - win_w, screen_size.y - win_h)
            }
        };

        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(pos));
    }

    fn handle_click(&self, ctx: &egui::Context, action: ClickAction) {
        match action {
            ClickAction::Copy => self.copy_to_clipboard(),
            ClickAction::CopyClose => {
                self.copy_to_clipboard();
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            ClickAction::Close => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
            ClickAction::Nothing => {}
        }
    }

    fn copy_to_clipboard(&self) {
        let w = self.img_size[0] as usize;
        let h = self.img_size[1] as usize;
        let mut clipboard = Clipboard::new().unwrap();
        let img_data = arboard::ImageData {
            width: w,
            height: h,
            bytes: self.rgba_for_copy.as_slice().into(),
        };
        clipboard.set_image(img_data).ok();
    }
}

fn main() -> eframe::Result {
    let args = Args::parse();

    if !args.clipboard && args.image_path.is_none() {
        eprintln!("error: 需要提供图片路径或使用 --clipboard");
        std::process::exit(1);
    }

    let (rgba, img_size) = if args.clipboard {
        let mut clipboard = Clipboard::new().expect("无法访问剪贴板");
        let img_data = clipboard.get_image().expect("剪贴板中没有图片");
        let w = img_data.width;
        let h = img_data.height;
        let rgba: Vec<u8> = img_data.bytes.into();
        (rgba, [w as u32, h as u32])
    } else {
        let path = args.image_path.as_ref().unwrap();
        let img = image::open(path).expect("无法加载图片");
        let (w, h) = img.dimensions();
        let rgba = img.to_rgba8();
        (rgba.into_raw(), [w, h])
    };

    let img_size = match (args.width, args.height) {
        (Some(w), Some(h)) => [w, h],
        _ => img_size,
    };

    let viewport = egui::ViewportBuilder::default()
        .with_decorations(false)
        .with_always_on_top()
        .with_transparent(true)
        .with_resizable(false)
        .with_inner_size(egui::Vec2::new(img_size[0] as f32, img_size[1] as f32));

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "pin-image",
        options,
        Box::new(move |cc| {
            let app = PinImage::new(
                &cc.egui_ctx,
                rgba,
                img_size,
                args.location,
                args.left_click,
                args.right_click,
                args.double_click,
            );
            Ok(Box::new(app))
        }),
    )
}
