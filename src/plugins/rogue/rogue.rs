use bevy::prelude::*;

use crate::{
    plugins::{
        actions::skill_id::SkillId,
        area::PlayerEnterEvent,
        player::Hero,
        rogue::shop::{refresh_shop, RefreshShopEvent, ShopSlot, SlotAction},
        spider::spawn_spider,
        unit::{Unit, UnitDieEvent},
        units::unit_command::UnitCommand,
    },
    AppState,
};

use super::shop::{
    buy_action, hero_get_action, shop_slot_changed, slot_action_changed, slot_detection,
    slot_item_changed, slot_ui, HeroGetAction, ShopSlotBuyEvent,
};

pub struct RoguePlugin;
impl Plugin for RoguePlugin {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<LevelEnemy>()
            .add_system(spawn_enemy)
            .add_system(enemy_counting)
            .add_system(start_level)
            .add_event::<BackToEvent>()
            .add_system(back_to)
            // Shop
            .register_type::<ShopSlot>()
            .register_type::<SlotAction>()
            .add_event::<RefreshShopEvent>()
            .add_system(refresh_shop)
            // .add_system(attach_shop_item)
            .add_system(shop_slot_changed)
            .add_system(slot_item_changed)
            .add_system(slot_action_changed)
            .add_system(slot_detection)
            .add_event::<ShopSlotBuyEvent>()
            .add_system(slot_ui)
            .add_system(buy_action)
            .add_event::<HeroGetAction>()
            .add_system(hero_get_action)
            // .init_resource::<RogueRes>()
            .insert_resource(RogueRes {
                level: 0,
                counter: 0,
                remain: 0,
                started: false,
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            })
            .add_system_set(SystemSet::on_enter(AppState::Level).with_system(
                |mut refresh_events: EventWriter<RefreshShopEvent>| {
                    refresh_events.send_default();
                },
            ));
    }
}

#[derive(Debug, Default, Resource)]
pub struct RogueRes {
    pub level: i32,
    pub counter: i32,
    pub remain: i32,
    pub started: bool,
    pub timer: Timer,
}

impl RogueRes {
    pub fn start_next_level(&mut self) {
        self.level += 1;
        self.counter = self.level;
        self.remain = self.level;
        // self.counter = 5 + self.level;
        // self.remain = 5 + self.level;
        self.started = true;
        self.timer.reset();
    }
}

pub fn spawn_enemy(
    mut commands: Commands,
    time: Res<Time>,
    mut rogue_res: ResMut<RogueRes>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    player_q: Query<&GlobalTransform, With<Hero>>,
) {
    if !rogue_res.started {
        return;
    }
    rogue_res.timer.tick(time.delta());
    if rogue_res.timer.just_finished() {
        if let Ok(player) = player_q.get_single() {
            if rogue_res.counter > 0 {
                let id = spawn_spider(
                    &mut commands,
                    player.translation().truncate(),
                    &asset_server,
                    &mut texture_atlases,
                );
                commands.entity(id).insert(LevelEnemy);
                rogue_res.counter -= 1;
            }
        }
    }
}

#[derive(Debug, Clone, Component, Reflect)]
struct LevelEnemy;

fn enemy_counting(
    mut events: EventReader<UnitDieEvent>,
    mut rogue_res: ResMut<RogueRes>,
    q: Query<Entity, With<LevelEnemy>>,
    mut back_events: EventWriter<BackToEvent>,
    mut refresh_events: EventWriter<RefreshShopEvent>,
    mut commands: Commands,
) {
    for ev in events.iter() {
        if q.get(ev.0).is_ok() {
            rogue_res.remain -= 1;
            info!("enemy remain: {}", rogue_res.remain);
            if rogue_res.remain == 0 {
                info!("Finish level");
                back_events.send_default();
                refresh_events.send_default();
                for e in q.iter() {
                    commands.entity(e).despawn_recursive();
                }
            }
        }
    }
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct StartTeleport;
#[derive(Debug, Clone, Component, Reflect)]
pub struct StartTeleportTarget;

pub fn start_level(
    mut enter_events: EventReader<PlayerEnterEvent>,
    tele_q: Query<&StartTeleport>,
    mut player_query: Query<
        (
            &mut Transform,
            &mut UnitCommand,
            &GlobalTransform,
            &mut Unit,
        ),
        With<Hero>,
    >,
    target_query: Query<&GlobalTransform, With<StartTeleportTarget>>,
    mut rogue_res: ResMut<RogueRes>,
) {
    for ev in enter_events.iter() {
        if tele_q.get(ev.0).is_ok() {
            info!("Receive PlayerEnterEvent for StartTeleport");
            if let Ok(target) = target_query.get_single() {
                if let Ok((mut player, mut uc, player_global, mut unit)) =
                    player_query.get_single_mut()
                {
                    let t = target.translation() - player_global.translation() + player.translation;
                    player.translation.x = t.x;
                    player.translation.y = t.y;
                    uc.action_id = SkillId::Idle;
                    unit.hp = unit.hp_max;
                    rogue_res.start_next_level();
                } else {
                    error!("No player to teleport");
                }
            } else {
                error!("Cannot get StartTeleportTarget");
            }
        }
    }
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct BackTo;

#[derive(Debug, Clone, Default)]
pub struct BackToEvent;
fn back_to(
    mut events: EventReader<BackToEvent>,
    mut player_query: Query<
        (
            &mut Transform,
            &mut UnitCommand,
            &GlobalTransform,
            &mut Unit,
            &mut Hero,
        ),
        With<Hero>,
    >,
    target_query: Query<&GlobalTransform, With<BackTo>>,
) {
    for _ in events.iter() {
        if let Ok(target) = target_query.get_single() {
            if let Ok((mut player, mut uc, player_global, mut unit, mut hero)) =
                player_query.get_single_mut()
            {
                let t = target.translation() - player_global.translation() + player.translation;
                player.translation.x = t.x;
                player.translation.y = t.y;
                uc.action_id = SkillId::Idle;
                unit.hp = unit.hp_max;
                hero.gold += 100;
            } else {
                error!("No player to go back");
            }
        } else {
            error!("Cannot get BackTo");
        }
    }
}

// #[derive(Debug, Clone, Component, Reflect)]
// pub struct FirstWeapon;
// pub fn pick_first_weapon(
//     mut refresh_events: EventWriter<RefreshShopEvent>,
//     //
// ) {
//     refresh_events.send_default();
// }
