use bevy::prelude::*;

use crate::{
    plugins::team::Team,
    utils::{Knockback, Shape},
};

pub fn spawn_attack(
    commands: &mut Commands,
    position: Transform,
    player: Entity,
    team: &Team,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Entity {
    let animation_entity = {
        let texture_handle = asset_server.load("images/player/spritesheet.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 10, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite {
                    index: 9,
                    ..Default::default()
                },
                ..Default::default()
            })
            .id()
    };

    let id = crate::utils::spawn_melee(
        crate::utils::Melee {
            offset: Vec2::new(1.0, 0.0),
            lifespan: 0.1,
            source: player,
            shape: Shape::Cuboid(0.5, 0.5),
            target_team: team.enemy_target(),
            parent_position: position,
            damage: 1,
            hit_stun: 0.3,
            knockback: Knockback::Direction(0.2, 0.0),
        },
        commands,
    );
    commands.entity(id).add_child(animation_entity);
    id
}
