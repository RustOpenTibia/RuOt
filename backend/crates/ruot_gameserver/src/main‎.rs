use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*};
use core::time::Duration;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 50.0)))
        )
        .add_systems(Update, counter)
        .run();
}

fn counter(mut state: Local<CounterState>) {
    if state.count % 60 == 0 {
        println!("{}", state.count);
    }
    state.count += 1;
}

#[derive(Default)]
struct CounterState {
    count: u32,
}
