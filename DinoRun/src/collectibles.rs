use crate::constants::*;
use crate::types::CollectibleKind;
use macroquad::{prelude::*, rand::gen_range};

/// 场景中的可收集物体，包含漂浮动画相位。
pub struct Collectible {
    pub kind: CollectibleKind,
    pub rect: Rect,
    pub value: u32,
    pub float_phase: f32,
}

impl Collectible {
    /// 利用随机数对生成位置与价值进行控制。
    pub fn new(kind: CollectibleKind, ground_y: f32) -> Self {
        match kind {
            CollectibleKind::Coin => {
                let size = 24.0;
                let x = SCREEN_WIDTH + gen_range(40.0, 200.0);
                let y = ground_y - PLAYER_SIZE.y - gen_range(40.0, 130.0);
                Self {
                    kind,
                    rect: Rect::new(x, y, size, size),
                    value: 20,
                    float_phase: gen_range(0.0, 360.0),
                }
            }
            CollectibleKind::Gem => {
                let size = 30.0;
                let x = SCREEN_WIDTH + gen_range(240.0, 380.0);
                let y = ground_y - PLAYER_SIZE.y - gen_range(70.0, 220.0);
                Self {
                    kind,
                    rect: Rect::new(x, y, size, size),
                    value: 120,
                    float_phase: gen_range(0.0, 360.0),
                }
            }
        }
    }

    /// 更新位置与漂浮相位，响应整体卷轴速度。
    pub fn update(&mut self, dt: f32, speed: f32) {
        self.rect.x -= speed * dt;
        self.float_phase += dt * 3.0;
    }

    /// 判断物体是否完全离开屏幕，用于回收。
    pub fn is_offscreen(&self) -> bool {
        self.rect.x + self.rect.w < -60.0
    }
}

/// 根据类型绘制收集物，并加入轻微浮动效果。
pub fn draw_collectibles(collectibles: &[Collectible]) {
    for item in collectibles {
        let mut y = item.rect.y;
        y += item.float_phase.sin() * 10.0;
        let color = match item.kind {
            CollectibleKind::Coin => Color::from_rgba(255, 210, 64, 255),
            CollectibleKind::Gem => Color::from_rgba(90, 210, 255, 255),
        };
        draw_rectangle(item.rect.x, y, item.rect.w, item.rect.h, color);
        draw_rectangle_lines(
            item.rect.x,
            y,
            item.rect.w,
            item.rect.h,
            2.0,
            Color::from_rgba(0, 0, 0, 160),
        );
    }
}
