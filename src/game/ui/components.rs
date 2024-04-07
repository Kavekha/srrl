use bevy::prelude::*;

use crate::game::combat::components::AttackType;


// Toute UI in Game utilise ce tag.
#[derive(Component)]
pub struct UiGameInterface;

#[derive(Component)]
pub struct UiCharacterInfos;


#[derive(Component)]
pub struct UiEnemyHp;


#[derive(Component)]
pub struct UiActionPointsOnCursor;

#[derive(Component)]
pub struct UiLog;

#[derive(Component)]
pub struct UiAttackIcon{
    pub attack_type: AttackType
}

#[derive(Component)]
pub struct UiMainWindow;

