//! 游戏程序入口，负责初始化资源、驱动主循环并协调输入、逻辑与渲染模块。

mod collectibles;
mod constants;
mod input;
mod obstacles;
mod particles;
mod player;
mod render;
mod types;
mod update;
mod utils;
mod world;

use constants::*;
use input::handle_input;
use macroquad::prelude::*;
use render::draw_world;
use update::update_world;
use world::World;

#[macroquad::main(window_conf)]
async fn main() {
    // 加载玩家贴图并使用最近邻采样保持像素风格。
    let player_texture: Texture2D = load_texture("assets/player.png").await.unwrap();
    player_texture.set_filter(FilterMode::Nearest);

    // World 结构体承载游戏状态，贯穿整个生命周期。
    let mut world = World::new(player_texture);

    // 主循环：处理输入、更新逻辑与渲染输出，随后等待下一帧。
    loop {
        let dt = get_frame_time();
        handle_input(&mut world, dt);
        update_world(&mut world, dt);
        draw_world(&world);
        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        // 配置窗口标题与尺寸，确保与常量定义保持一致。
        window_title: "Neon Run".to_string(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        high_dpi: true,
        sample_count: 4,
        ..Default::default()
    }
}
