#![doc = include_str!("../README.md")]
use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::query::{Changed, Or, With};
use bevy_ecs::system::{Commands, NonSend, Query};
use bevy_window::{Monitor, Window};
use bevy_winit::WinitWindows;

/// The main plugin that, when added, will automagically update and manage Window positions.
pub struct LinkWindowToMonitorPlugin;

impl Plugin for LinkWindowToMonitorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, link_monitor_to_window);
    }
}

#[derive(Component, Debug)]
#[relationship(relationship_target = HasWindows)]
pub struct OnMonitor(pub Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = OnMonitor, linked_spawn)]
pub struct HasWindows(Vec<Entity>);

fn link_monitor_to_window(
    mut commands: Commands,
    windows: Query<
        (Entity, Option<&OnMonitor>),
        (With<Window>, Or<(Changed<Monitor>, Changed<Window>)>),
    >,
    monitors: Query<(&Monitor, Entity)>,
    underlying: NonSend<WinitWindows>,
) {
    'window: for (entity, on_monitor) in windows {
        let winit_monitor = match underlying
            .entity_to_winit
            .get(&entity)
            .and_then(|i| underlying.windows.get(i))
            .and_then(|w| w.current_monitor())
        {
            None => {
                commands.entity(entity).remove::<OnMonitor>();
                continue;
            }
            Some(monitor) => monitor,
        };

        for (monitor, monitor_entity) in monitors {
            if monitor.name != winit_monitor.name()
                || monitor.physical_position.x != winit_monitor.position().x
                || monitor.physical_position.y != winit_monitor.position().y
            {
                continue;
            }

            if on_monitor.is_none_or(|m| m.0 != monitor_entity) {
                commands.entity(entity).insert(OnMonitor(monitor_entity));
            }
            continue 'window;
        }
        commands.entity(entity).remove::<OnMonitor>();
    }
}
