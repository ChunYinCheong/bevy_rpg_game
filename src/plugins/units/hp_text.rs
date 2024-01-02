use bevy::{math::Affine3A, prelude::*};

use super::unit::Unit;

pub fn attach_text(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    unit_q: Query<(Entity, &Unit), Added<Unit>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    for (entity, unit) in unit_q.iter() {
        command.entity(entity).with_children(|builder| {
            builder
                .spawn(Text2dBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: " \n".into(),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size: 24.0,
                                    color: Color::GREEN,
                                },
                            },
                            TextSection {
                                value: format!("{}", unit.hp),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size: 24.0,
                                    color: Color::GREEN,
                                },
                            },
                        ],
                        alignment: TextAlignment::TOP_CENTER,
                    },

                    ..Default::default()
                })
                .insert(HpText(entity));
        });
        // command
        //     .spawn(Text2dBundle {
        //         text: Text {
        //             sections: vec![TextSection {
        //                 value: format!("Hp: {}", unit.hp),
        //                 style: TextStyle {
        //                     font: font.clone(),
        //                     font_size: 24.0,
        //                     color: Color::GREEN,
        //                 },
        //             }],
        //             ..Default::default()
        //         },
        //         ..Default::default()
        //     })
        //     .insert(RemoteTransform(entity))
        //     .insert(HpText(entity))
        //     .insert(Name::new(format!("HpText({entity:?})")));
    }
}

// #[derive(Debug, Clone, Component, Reflect)]
// pub struct RemoteTransform(pub Entity);

#[derive(Debug, Clone, Component, Reflect)]
pub struct HpText(pub Entity);

pub fn update_hp_text(
    unit_q: Query<&Unit, Changed<Unit>>,
    mut text_q: Query<(&mut Text, &HpText)>,
) {
    for (mut text, entity) in text_q.iter_mut() {
        if let Ok(unit) = unit_q.get(entity.0) {
            if let Some(t) = text.sections.get_mut(1) {
                t.value = format!("{}", unit.hp);
            }
        }
    }
}

pub fn fix_rotation(mut remote_q: Query<(&mut GlobalTransform, &HpText)>) {
    for (mut transform, _) in remote_q.iter_mut() {
        *transform = GlobalTransform::from(Affine3A::from_translation(transform.translation()));
    }
}

// pub fn update_remote_transform(
//     mut remote_q: Query<(&mut Transform, &RemoteTransform)>,
//     target_q: Query<&GlobalTransform>,
// ) {
//     for (mut t, r) in remote_q.iter_mut() {
//         if let Ok(target) = target_q.get(r.0) {
//             t.translation = target.translation();
//         }
//     }
// }

// pub fn clear_text(
//     mut command: Commands,
//     removed_q: RemovedComponents<Unit>,
//     text_q: Query<&HpText>,
// ) {
//     for id in removed_q.iter() {
//         text_q.iter().filter(|text| text.0 == id).for_each(|text| {
//             command.entity(text.0).despawn_recursive();
//         })
//     }
// }
