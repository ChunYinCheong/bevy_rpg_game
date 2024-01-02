use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::team::Team;

pub struct HitPlugin;
impl Plugin for HitPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .add_system(hit_detection)
            .register_type::<Hit>()
            .add_event::<HitEvent>();
    }
}

#[derive(Debug, Component, Reflect)]
pub struct Hit {
    pub target_team: Team,
}

pub struct HitEvent {
    pub hit_entity: Entity,
    pub target_entity: Entity,
}

pub fn hit_detection(
    mut collision_events: EventReader<CollisionEvent>,
    mut events: EventWriter<HitEvent>,
    query: Query<&Hit>,
    team_q: Query<&Team>,
) {
    for collision_event in collision_events.iter() {
        // info!("Received collision event: {:?}", collision_event);
        match collision_event {
            CollisionEvent::Started(e1, e2, _flag) => {
                let r = {
                    let mut temp = None;
                    if let Ok(hit) = query.get(*e1) {
                        if let Ok(team) = team_q.get(*e2) {
                            if team == &hit.target_team {
                                temp = Some((*e2, *e1));
                            }
                        }
                    } else if let Ok(hit) = query.get(*e2) {
                        if let Ok(team) = team_q.get(*e1) {
                            if team == &hit.target_team {
                                temp = Some((*e1, *e2));
                            }
                        }
                    }
                    temp
                };
                if let Some((victim, source_collider)) = r {
                    events.send(HitEvent {
                        hit_entity: source_collider,
                        target_entity: victim,
                    });
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}
