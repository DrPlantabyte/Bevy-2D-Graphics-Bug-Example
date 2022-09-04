use bevy::prelude::*;
use bevy::app::AppExit;
use std::thread;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_startup_system_to_stage(StartupStage::Startup, setup)
		.add_startup_system_to_stage(StartupStage::PostStartup, start)
		.add_system(exit_on_all_closed)
		.run();
}

const BACKGROUND_COLOR: Color = Color::rgb(0.7, 0.7, 0.8);
struct MyResource{
	tex: Handle<Image>
}


fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>
) {
	commands.insert_resource(ClearColor(BACKGROUND_COLOR));
	commands.insert_resource(MyResource{tex: asset_server.load("boot.png")});
	commands.spawn_bundle(Camera2dBundle::default());
}


fn start(mut commands: Commands, tex: Res<MyResource>) {
	commands
		.spawn_bundle(SpriteBundle{
			texture: tex.tex.clone(),
			..default()
		});
}

fn exit_on_all_closed(mut app_exit_events: EventWriter<AppExit>, windows: Res<Windows>) {
	// workaround to issue 5524: https://github.com/bevyengine/bevy/issues/5524
	let hundred_millis = std::time::Duration::from_millis(100);
	if windows.iter().count() == 0 {
		app_exit_events.send(AppExit);
		thread::sleep(hundred_millis);
		std::process::exit(0);
	}
}
