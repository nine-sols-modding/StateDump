use std::{borrow::Cow, collections::BTreeMap, marker::PhantomData};

use crate::attack_fsm::id_name;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionWeightQueue<T> {
    #[serde(rename = "IsRandom")]
    pub is_random: bool,
    state_weight_list: Vec<Weight<T>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Weight<T> {
    pub option: T,
    pub weight: i32,
}
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AttackWeight {
    pub state_type: State,
    pub weight: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct MonsterStateGroupSequence {
    #[serde(rename = "$id")]
    pub id: String,
    #[serde(rename = "AttackSequence")]
    pub attack_sequence: Vec<OptionWeightQueue<Reference<BossGeneralState>>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Reference<T> {
    #[serde(rename = "$ref")]
    pub reference: String,
    #[serde(skip)]
    pub _marker: PhantomData<T>,
}

#[derive(Debug, serde::Deserialize, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct State(pub String);

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkNextMoveStateWeight {
    pub r#type: String,
    #[serde(rename = "IsRandom")]
    pub is_random: bool,
    pub must_use_states: Vec<Reference<BossGeneralState>>,
    pub state_weight_list: Vec<AttackWeight>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateObject {
    #[serde(rename = "$id")]
    pub id: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnimationEvent {
    pub time: f32,
    pub event: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkMoveGroupNode {
    pub queue: MonsterStateQueue,
}
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonsterStateQueue {
    pub link_next_move_state_weights: Vec<LinkNextMoveStateWeight>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BossGeneralState {
    #[serde(rename = "$id")]
    pub id: String,
    // state: State,
    pub linked_state_types: Vec<State>,
    // indexed by phase index
    pub link_next_move_state_weights: Vec<LinkNextMoveStateWeight>,
    pub grouping_nodes: Vec<LinkMoveGroupNode>,
    pub exit_state: State,
    #[serde(default)]
    pub clip: Option<Vec<AnimationEvent>>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AttackSensor {
    #[serde(rename = "$id")]
    pub id: String,
    pub attack_weight_phase_list: Vec<LinkNextMoveStateWeight>,
    pub binding_attacks: Vec<State>,
    #[serde(rename = "_conditions")]
    pub conditions: Vec<serde_json::Value>,
    #[serde(rename = "forStateType")]
    pub for_state_type: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    // TODO no option
    pub init_state: State,
    #[serde(default)]
    pub phase_sequences: Vec<OptionWeightQueue<MonsterStateGroupSequence>>,
    #[serde(default)]
    pub attack_sensors: Vec<AttackSensor>,
    pub boss_general_states: BTreeMap<State, BossGeneralState>,
    pub other_states: BTreeMap<State, StateObject>,
}

impl Data {
    pub fn get_bgs(&self, id: &str) -> Option<&BossGeneralState> {
        self.boss_general_states.values().find(|x| x.id == id)
    }
    pub fn find_mapping_state_id(&self, state: &State) -> Option<Cow<str>> {
        self.boss_general_states
            .get(state)
            .map(|x| id_name(&x.id))
            .or_else(|| self.other_states.get(state).map(|x| id_name(&x.id)))
    }
}
