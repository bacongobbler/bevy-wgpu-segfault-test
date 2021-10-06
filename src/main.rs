use bevy::prelude::*;
use bevy_wgpu::WgpuPlugin;

fn main() {
    App::build()
        .add_plugin(WgpuPlugin)
        .run();
}
