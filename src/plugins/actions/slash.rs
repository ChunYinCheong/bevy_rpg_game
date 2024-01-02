use bevy::prelude::*;

use crate::plugins::{
    actions::spawn_attack,
    animation::ChangeAnimation,
    movement::Movement,
    team::Team,
    unit_action::UnitAnimation,
    unit_state::{ActionSystemLabel, UnitActionEnterActiveEvent, UnitActionEnterEvent},
};

use super::{action::Skill, base::BaseSkill};

pub struct SlashPlugin;
impl Plugin for SlashPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<SlashAction>()
            .add_system(attach)
            .add_system(enter.label(ActionSystemLabel::Enter))
            .add_system(enter_active.label(ActionSystemLabel::EnterActive));
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct SlashAction {}

fn attach(mut commands: Commands, q: Query<(Entity, &Skill), Added<Skill>>) {
    for (e, a) in q.iter() {
        let setting = a.action_id.setting();
        if let BaseSkill::Slash = setting.base {
            commands.entity(e).insert(SlashAction {});
        }
    }
}

fn enter(
    mut events: EventReader<UnitActionEnterEvent>,
    query: Query<(&SlashAction,)>,
    mut unit_q: Query<(&mut Movement,)>,
    mut anim_events: EventWriter<ChangeAnimation>,
) {
    for ev in events.iter() {
        if let Ok((_,)) = query.get(ev.action) {
            if let Ok((mut movement,)) = unit_q.get_mut(ev.unit) {
                movement.speed = 0.0;
                anim_events.send(ChangeAnimation {
                    entity: ev.unit,
                    name: UnitAnimation::Attack.to_string(),
                });
            }
        }
    }
}

fn enter_active(
    mut events: EventReader<UnitActionEnterActiveEvent>,
    query: Query<(&SlashAction,)>,
    unit_q: Query<(&GlobalTransform, &Team)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for ev in events.iter() {
        if let Ok((_,)) = query.get(ev.action) {
            debug!("Slash enter active: {:?}", ev);
            if let Ok((position, team)) = unit_q.get(ev.unit) {
                spawn_attack::spawn_attack(
                    &mut commands,
                    Transform::from(*position),
                    ev.unit,
                    team,
                    &asset_server,
                    &mut texture_atlases,
                );
            }
        }
    }
}
