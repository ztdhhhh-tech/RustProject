use crate::constants::*;
use crate::types::ObstacleKind;
use crate::utils::RectExt;
use macroquad::{prelude::*, rand::gen_range};

/// 运行时障碍物实体，包含碰撞盒与附加动画信息。
pub struct Obstacle {
    pub kind: ObstacleKind,
    pub rect: Rect,
    pub hurt_box: Rect,
    pub saw_angle: f32,
}

impl Obstacle {
    /// 根据障碍类别创建对应的几何形状与碰撞盒。
    pub fn new(kind: ObstacleKind, ground_y: f32) -> Self {
        match kind {
            ObstacleKind::Crate => {
                let width = gen_range(52.0, 92.0);
                let height = gen_range(56.0, 98.0);
                let x = SCREEN_WIDTH + width + gen_range(0.0, 120.0);
                let y = ground_y - height;
                let rect = Rect::new(x, y, width, height);
                Self {
                    kind,
                    hurt_box: rect,
                    rect,
                    saw_angle: 0.0,
                }
            }
            ObstacleKind::Saw => {
                let size = gen_range(66.0, 90.0);
                let x = SCREEN_WIDTH + size + gen_range(0.0, 160.0);
                let y = ground_y - size + gen_range(-12.0, 12.0);
                let rect = Rect::new(x, y, size, size);
                Self {
                    kind,
                    hurt_box: rect.inflate(-12.0, -12.0),
                    rect,
                    saw_angle: gen_range(0.0, 360.0),
                }
            }
            ObstacleKind::Pit => {
                let width = gen_range(140.0, 260.0);
                let x = SCREEN_WIDTH + width + gen_range(40.0, 190.0);
                let rect = Rect::new(x, ground_y - 4.0, width, 32.0);
                Self {
                    kind,
                    hurt_box: rect,
                    rect,
                    saw_angle: 0.0,
                }
            }
            ObstacleKind::Drone => {
                let width = 78.0;
                let height = 48.0;
                let x = SCREEN_WIDTH + width + gen_range(0.0, 160.0);
                let y = ground_y - PLAYER_SIZE.y - gen_range(120.0, 210.0);
                let rect = Rect::new(x, y, width, height);
                Self {
                    kind,
                    hurt_box: rect.inflate(-10.0, -10.0),
                    rect,
                    saw_angle: 0.0,
                }
            }
        }
    }

    /// 按时间推进障碍的运动与动画。
    pub fn update(&mut self, dt: f32, speed: f32) {
        self.rect.x -= speed * dt;
        self.hurt_box.x -= speed * dt;
        if self.kind == ObstacleKind::Saw {
            self.saw_angle += 6.4 * dt;
        }
        if self.kind == ObstacleKind::Drone {
            self.rect.y += self.saw_angle.sin() * 22.0 * dt;
            self.hurt_box.y = self.rect.y + 8.0;
            self.hurt_box.x = self.rect.x + 8.0;
        }
    }

    /// 判断障碍是否完全离开屏幕，用于回收。
    pub fn is_offscreen(&self) -> bool {
        self.rect.x + self.rect.w < -200.0
    }
}

/// 依据游戏时间与随机数决定下一种障碍。
pub fn choose_obstacle_kind(time: f32) -> ObstacleKind {
    let t = time.min(180.0);
    let roll = gen_range(0.0, 1.0);
    if roll < 0.4 {
        ObstacleKind::Crate
    } else if roll < 0.68 {
        ObstacleKind::Saw
    } else if t > 20.0 && roll < 0.88 {
        ObstacleKind::Pit
    } else {
        ObstacleKind::Drone
    }
}

/// 绘制场景内所有障碍，根据类型采用不同的外观表现。
pub fn draw_obstacles(obstacles: &[Obstacle]) {
    for obstacle in obstacles {
        match obstacle.kind {
            ObstacleKind::Crate => {
                draw_rectangle(
                    obstacle.rect.x,
                    obstacle.rect.y,
                    obstacle.rect.w,
                    obstacle.rect.h,
                    Color::from_rgba(120, 83, 58, 255),
                );
                draw_rectangle_lines(
                    obstacle.rect.x,
                    obstacle.rect.y,
                    obstacle.rect.w,
                    obstacle.rect.h,
                    3.0,
                    BLACK,
                );
            }
            ObstacleKind::Saw => {
                let center = vec2(
                    obstacle.rect.x + obstacle.rect.w * 0.5,
                    obstacle.rect.y + obstacle.rect.h * 0.5,
                );
                let radius = obstacle.rect.w * 0.5;
                draw_circle(
                    center.x,
                    center.y,
                    radius,
                    Color::from_rgba(210, 210, 210, 255),
                );
                for i in 0..8 {
                    let angle = obstacle.saw_angle + i as f32 * std::f32::consts::PI / 4.0;
                    let dir = vec2(angle.cos(), angle.sin());
                    draw_triangle(
                        center,
                        center + dir * radius,
                        center + vec2(-dir.y, dir.x) * (radius * 0.7),
                        Color::from_rgba(180, 180, 180, 255),
                    );
                }
            }
            ObstacleKind::Pit => {
                draw_rectangle(
                    obstacle.rect.x,
                    obstacle.rect.y,
                    obstacle.rect.w,
                    12.0,
                    Color::from_rgba(10, 20, 30, 255),
                );
                draw_rectangle(
                    obstacle.rect.x,
                    obstacle.rect.y + 12.0,
                    obstacle.rect.w,
                    16.0,
                    Color::from_rgba(12, 16, 18, 255),
                );
            }
            ObstacleKind::Drone => {
                draw_rectangle(
                    obstacle.rect.x,
                    obstacle.rect.y,
                    obstacle.rect.w,
                    obstacle.rect.h,
                    Color::from_rgba(90, 90, 110, 255),
                );
                draw_rectangle(
                    obstacle.rect.x + obstacle.rect.w * 0.1,
                    obstacle.rect.y + obstacle.rect.h * 0.2,
                    obstacle.rect.w * 0.8,
                    obstacle.rect.h * 0.6,
                    Color::from_rgba(40, 200, 200, 200),
                );
            }
        }
    }
}
