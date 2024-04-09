use bevy::prelude::*;


#[derive(Debug)]
pub enum GoalType{
    KillEntity { id: Entity },
    None,
}


#[derive(Component)]
pub struct Goal {
    pub id: GoalType,
}


// Le NPC doit considerer son objectif.
#[derive(Component)]
pub struct CheckGoal;

// Necessaire pour que le NPC sache qu'il doit plannifier.
#[derive(Component)]
pub struct Planning;

#[derive(Component)]
pub struct PlanMove;

/*

#[derive(Component)]
pub struct PlanHitMelee {
    target: Entity, 
    ap_cost: u32,
    weight: u32
}

#[derive(Component)]
pub struct PlanForfeitTurn {
    target: Entity,
    ap_cost: u32,
    weight: u32
}
 */