use macroquad::prelude::*;

/// 为 `Rect` 提供常用的扩展方法，方便碰撞检测调整。
pub trait RectExt {
    fn overlaps(&self, other: &Rect) -> bool;
    fn inflate(&self, amount_x: f32, amount_y: f32) -> Rect;
}

impl RectExt for Rect {
    fn overlaps(&self, other: &Rect) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }

    fn inflate(&self, amount_x: f32, amount_y: f32) -> Rect {
        Rect::new(
            self.x - amount_x,
            self.y - amount_y,
            self.w + amount_x * 2.0,
            self.h + amount_y * 2.0,
        )
    }
}

/// 平滑的二次缓出曲线，常用于淡入淡出动画。
pub fn ease_out_quad(t: f32) -> f32 {
    1.0 - (1.0 - t) * (1.0 - t)
}

/// 以文本宽度为基础计算居中位置，简化 UI 调用。
pub fn draw_text_centered(text: &str, x: f32, y: f32, size: f32, color: Color) {
    let dims = measure_text(text, None, size as u16, 1.0);
    draw_text(text, x - dims.width * 0.5, y, size, color);
}
