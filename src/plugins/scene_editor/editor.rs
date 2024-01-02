use bevy::prelude::*;

use crate::plugins::game_world::{GameObjectId, GameObjectType};

use super::scene_loader::SceneRes;

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            // .add_startup_system(init_scene.after(load))
            .add_system(update);
    }
}

// fn init_scene(mut commands: Commands, asset_server: Res<AssetServer>, scene_res: Res<SceneRes>) {
//     let font = asset_server.load("fonts/FiraSans-Bold.ttf");
//     let text_style = TextStyle {
//         font,
//         font_size: 24.0,
//         color: Color::WHITE,
//     };
//     let text_alignment = TextAlignment::CENTER;

//     let keys = scene_res.ecs.objects.keys().cloned();
//     for id in keys {
//         let entity = commands
//             .spawn(Text2dBundle {
//                 text: Text::from_section(&id.0, text_style.clone()).with_alignment(text_alignment),
//                 ..Default::default()
//             })
//             .insert(id.clone())
//             .id();

//         if let Some(v) = scene_res.ecs.transforms.get(&id) {
//             commands.entity(entity).insert(Transform {
//                 translation: {
//                     let mut t = v.0;
//                     t.z = 1.0;
//                     t
//                 },
//                 ..Default::default()
//             });
//         }
//     }
// }

fn update(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scene_res: Res<SceneRes>,
    query: Query<&GameObjectId>,
    mut transforms: Query<(&GameObjectId, &mut Transform)>,
    mut objects: Query<(&GameObjectId, &mut GameObjectType)>,
) {
    if scene_res.is_changed() {
        //
        let keys = scene_res.ecs.objects.keys().cloned();

        for id in keys {
            if query.iter().any(|goid| goid == &id) {
                continue;
            }
            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
            let text_style = TextStyle {
                font,
                font_size: 24.0,
                color: Color::WHITE,
            };
            let text_alignment = TextAlignment::CENTER;

            let entity = commands
                .spawn(Text2dBundle {
                    text: Text::from_section(&id.0, text_style.clone())
                        .with_alignment(text_alignment),
                    ..Default::default()
                })
                .insert(id.clone())
                .insert(Name::new(id.0.clone()))
                .id();

            if let Some(v) = scene_res.ecs.transforms.get(&id) {
                commands.entity(entity).insert(Transform {
                    translation: v.0,
                    ..Default::default()
                });
            }
            if let Some(v) = scene_res.ecs.objects.get(&id) {
                commands.entity(entity).insert(v.clone());
            }
        }

        //
        for (id, mut tran) in transforms.iter_mut() {
            if let Some(v) = scene_res.ecs.transforms.get(id) {
                let mut t = v.0;
                t.z = 1.0;
                tran.translation = t;
            }
        }
        for (id, mut obj) in objects.iter_mut() {
            if let Some(v) = scene_res.ecs.objects.get(id) {
                *obj = *v;
            }
        }
    }
}
