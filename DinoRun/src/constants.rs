use macroquad::prelude::*;

// 画面配置 -----------------------------------------------------------------
// 游戏基准分辨率（像素），被窗口配置与渲染逻辑引用。
pub const SCREEN_WIDTH: f32 = 1100.0;
pub const SCREEN_HEIGHT: f32 = 620.0;

// 地面与玩家 -----------------------------------------------------------------
// 计算场景地板的基线位置以及玩家初始尺寸与出生坐标。
pub const BASE_GROUND_Y: f32 = SCREEN_HEIGHT - 112.0;
pub const PLAYER_SIZE: Vec2 = Vec2 { x: 56.0, y: 72.0 };
pub const PLAYER_START: Vec2 = Vec2 {
    x: 160.0,
    y: BASE_GROUND_Y - PLAYER_SIZE.y,
};

// 物理参数 -------------------------------------------------------------------
// 模拟重力、终端速度与冲刺水平速度，确保不同系统共用同一数值基准。
pub const GRAVITY: f32 = 2000.0;
pub const TERMINAL_VEL: f32 = 1400.0;
pub const DASH_SPEED: f32 = 820.0;

// 场景滚屏与地形 -------------------------------------------------------------
// 控制基础滚动速度、最高速度以及地面高度的波动范围，用于动态难度。
pub const BASE_SCROLL_SPEED: f32 = 360.0;
pub const MAX_SCROLL_SPEED: f32 = 820.0;
pub const GROUND_VARIATION: f32 = 40.0;

// 体力系统 -------------------------------------------------------------------
// 决定冲刺体力的上限以及消耗、恢复速率。
pub const MAX_STAMINA: f32 = 100.0;
pub const STAMINA_CONSUME_RATE: f32 = 48.0;
pub const STAMINA_RECOVER_RATE: f32 = 22.0;

// 游戏时序 -------------------------------------------------------------------
// 强化效果持续时间、昼夜循环长度以及菜单淡入时长。
pub const POWERUP_DURATION: f32 = 6.0;
pub const DAY_NIGHT_DURATION: f32 = 45.0;
pub const MENU_FADE_TIME: f32 = 1.4;

// 粒子系统 -------------------------------------------------------------------
// 预分配的粒子数量上限，用于粒子池初始化。
pub const MAX_PARTICLES: usize = 120;
