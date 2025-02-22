use std::marker::PhantomData;

use bevy::{
    ecs::{
        event::{Event, Events},
        system::Resource,
    },
    prelude::*,
};

use crate::gamestate::GameState;

pub struct ResendEventPlugin<T: Event> {
    _marker: PhantomData<T>,
}

impl<T: Event> Default for ResendEventPlugin<T> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T: Event> Plugin for ResendEventPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system(setup::<T>.in_schedule(OnEnter(GameState::Loading)))
            .add_system(enqueue_events::<T>.run_if(in_state(GameState::Loading)))
            .add_system(resend_events::<T>.in_schedule(OnEnter(GameState::Playing)))
            .add_system(cleanup::<T>.in_schedule(OnEnter(GameState::Playing)));
    }
}

#[derive(Resource)]
struct EventQueue<T: Event>(Vec<T>);

fn setup<T: Event>(mut commands: Commands) {
    commands.insert_resource(EventQueue::<T>(Vec::new()));
}

fn cleanup<T: Event>(mut commands: Commands) {
    commands.remove_resource::<EventQueue<T>>();
}

fn enqueue_events<T: Event>(mut queue: ResMut<EventQueue<T>>, mut events: ResMut<Events<T>>) {
    queue.0.extend(events.drain());
}

fn resend_events<T: Event>(
    mut commands: Commands,
    mut queue: ResMut<EventQueue<T>>,
    mut events: EventWriter<T>,
) {
    for event in queue.0.drain(..) {
        events.send(event);
    }
    commands.remove_resource::<EventQueue<T>>();
}
