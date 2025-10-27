use macroquad::prelude::*;

/// 简单的颜色插值工具，用于昼夜循环等渐变效果。
#[derive(Clone, Copy)]
pub struct ColorLerp {
    pub start: Color,
    pub end: Color,
}

impl ColorLerp {
    /// 按照参数 `t` 返回介于 start 与 end 之间的插值颜色。
    pub fn sample(&self, t: f32) -> Color {
        Color::new(
            self.start.r + (self.end.r - self.start.r) * t,
            self.start.g + (self.end.g - self.start.g) * t,
            self.start.b + (self.end.b - self.start.b) * t,
            1.0,
        )
    }
}

/// 玩家当前执行的动作，驱动动画、碰撞盒以及体力消耗逻辑。
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PlayerAction {
    Running,
    Jump,
    DoubleJump,
    Slide,
    Dash,
}

/// 强化道具的种类，用于决定触发的增益效果。
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PowerUpKind {
    Shield,
    ScoreBoost,
    TimeSlow,
}

/// 场景中可生成的障碍物类型。
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ObstacleKind {
    Crate,
    Saw,
    Pit,
    Drone,
}

/// 可收集物体的分类，与得分与货币奖励相关联。
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CollectibleKind {
    Coin,
    Gem,
}

/// 游戏状态机的枚举，涵盖了所有可见流程。
#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Splash { timer: f32 },
    Menu { fade: f32 },
    Running,
    Paused,
    GameOver { cooldown: f32 },
}

/// 视差层的参数集合，用于绘制背景带状图形。
#[derive(Clone, Copy)]
pub struct ParallaxLayer {
    pub height: f32,
    pub speed: f32,
    pub color: Color,
    pub thickness: f32,
}

/// 浮动提示文字，用于表现提示与得分反馈。
pub struct FloatingText {
    pub text: String,
    pub pos: Vec2,
    pub vel: Vec2,
    pub timer: f32,
    pub max_timer: f32,
    pub color: Color,
}

/// 动态难度曲线的状态结构，掌控障碍与滚屏节奏。
pub struct DifficultyTrack {
    pub time: f32,
    pub base_spacing: f32,
    pub rng_obstacle_timer: f32,
    pub rng_collectible_timer: f32,
}

impl DifficultyTrack {
    /// 使用默认值初始化曲线，适用于新开一局的初始节奏。
    pub fn new() -> Self {
        Self {
            time: 0.0,
            base_spacing: 1.4,
            rng_obstacle_timer: 1.6,
            rng_collectible_timer: 1.2,
        }
    }

    /// 依据时间推移提升滚动速度，直到达到上限。
    pub fn scroll_speed(&self) -> f32 {
        use crate::constants::*;
        (BASE_SCROLL_SPEED + self.time * 12.0).min(MAX_SCROLL_SPEED)
    }

    /// 让地面高度随时间轻微波动，营造灵动感。
    pub fn ground_y(&self) -> f32 {
        use crate::constants::*;
        BASE_GROUND_Y + (self.time * 0.1).sin() * GROUND_VARIATION
    }

    /// 返回下次障碍刷新的间隔，时间越久越短。
    pub fn obstacle_interval(&self) -> f32 {
        (self.base_spacing - self.time * 0.012).max(0.62)
    }

    /// 返回收集物的刷新间隔，带有周期性的呼吸感。
    pub fn collectible_interval(&self) -> f32 {
        (1.0 + (self.time * 0.027).sin()).max(0.3)
    }
}
