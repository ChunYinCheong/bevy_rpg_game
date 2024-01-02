pub mod action;
pub mod attack;
pub mod attack_aura;
pub mod base;
pub mod burning;
pub mod burst_fire;
pub mod dead;
pub mod dead_finger;
pub mod forbidden_array;
pub mod ghost_light;
pub mod heal_aura;
pub mod hook;
pub mod ice_spear;
pub mod idle;
pub mod move_to;
pub mod setting;
pub mod skill_id;
pub mod slash;
pub mod spawn_attack;
pub mod spider_attack;
pub mod stab;
pub mod stop;
pub mod stun;
pub mod wolf_attack;

// struct Ability {
//     // Unit or Item
//     // pub owner: bevy::prelude::Entity,
//     pub id: AbilityId,
//     pub level: i32,
//     pub cooldown: f32,
// }

// enum AbilityId {
//     TestAttack,
//     SuperFire,
//     SuperHeal,
// }

// impl AbilityId {
//     fn setting(&self) -> AbilitySetting {
//         match self {
//             _ => SUPER_FIRE,
//         }
//     }
// }

// struct AbilitySetting<'a> {
//     pub id: AbilityId,
//     pub name: &'a str,
//     pub desc: &'a str,
//     pub icon: &'a str,
//     pub base: BaseAbility,
// }

// enum BaseAbility {
//     Heal(Heal),
//     Fireball(Fireball),
// }

// struct Heal {
//     cooldown: [f32; 10],
//     mp_cost: [i32; 10],
//     healing: [i32; 10],
// }

// struct Fireball {
//     cooldown: [f32; 10],
//     mp_cost: [i32; 10],
//     damage: [i32; 10],
// }

// const SUPER_FIRE: AbilitySetting = AbilitySetting {
//     id: AbilityId::SuperFire,
//     name: "Super fire",
//     desc: "This is a very powerful fire ball!",
//     icon: "path to image file",
//     base: BaseAbility::Fireball(Fireball {
//         cooldown: [5.0; 10],
//         mp_cost: [10; 10],
//         damage: [10, 20, 30, 40, 50, 60, 70, 80, 90, 100],
//     }),
// };

// const player_ability_fireball: Ability = Ability {
//     id: AbilityId::SuperFire,
//     level: 1,
//     cooldown: 0.0,
//     // owner: todo!(),
// };
