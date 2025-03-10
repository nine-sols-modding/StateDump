use std::{borrow::Cow, fmt::Write as _, sync::LazyLock};

use crate::{
    dot::DotGraph,
    json::{Data, LinkNextMoveStateWeight},
};
use anyhow::Result;
use regex::Regex;

const COLOR_END: &str = "aquamarine3";
const COLOR_WEIGHTS: &str = "aquamarine4";
const COLOR_MUST_USE: &str = "goldenrod1";
const COLOR_EXIT: &str = "cornsilk3";

pub struct Context {
    pub data: Data,
}
impl Context {
    pub fn dump_graphviz(&self) -> Result<String> {
        let mut dg = DotGraph::new("Foo", "digraph", &[("rankdir", "LR")])
            .node_attributes(&[
                ("shape", "box"),
                ("fontname", "Helvetica"),
                // ("fontsize", "18"),
            ])
            .edge_attributes(&[("fontname", "Helvetica")]);

        for bgs in self.data.boss_general_states.values() {
            let node_id = id_name(&bgs.id);

            let clip = bgs.clip.as_deref().unwrap_or_default();
            let clip_text = clip
                .iter()
                .filter_map(|e| {
                    Some(format!(
                        "{:.2}s: {}",
                        e.time,
                        enum_value_only(e.event.as_deref()?)
                    ))
                })
                .collect::<Vec<_>>()
                .join("<br/>");

            self.add_detail_node(&mut dg, &node_id, &node_id, &clip_text);

            if let Some(exit_state) = self.data.find_mapping_state_id(&bgs.exit_state) {
                dg.add_edge(&node_id, &exit_state, &[
                    ("color", COLOR_EXIT),
                    ("fontcolor", COLOR_EXIT),
                ]);
            }
            let linked_states = bgs
                .linked_state_types
                .iter()
                .filter_map(|state| self.data.find_mapping_state_id(state));
            if let Some(table) = table_from_rows(linked_states, 0.0, 2.0) {
                let linked_id = format!("{}_linked", node_id);

                dg.add_node(&linked_id, &[
                    ("shape", "box"),
                    ("style", "dotted"),
                    ("label", &table),
                    ("margin", "0.05"),
                ]);

                dg.add_edge(&node_id, &linked_id, &[
                    ("label", "QueueEnd"),
                    ("color", COLOR_END),
                    ("fontcolor", COLOR_END),
                ]);
            }

            let phase_index = 0;
            if let Some(next_move_weights) = bgs.link_next_move_state_weights.get(phase_index) {
                self.link_next_move(&mut dg, &node_id, next_move_weights);
            }
        }

        /*let mut other = DotGraph::new("other", "subgraph", &[("rank", "sink")]);
        for state in data.other_states.values() {
            let name = id_name(&state.id);
            other.add_node(&id_name(&state.id), &[("label", &name)]);
            dg.add_edge(
                &data.boss_general_states.iter().last().unwrap().1.node_id(),
                &id_name(&state.id),
                &[("style", "invis")],
            );
        }
        dg.add_sub_graph(other);*/

        if let Some(init_state) = self.data.find_mapping_state_id(&self.data.init_state) {
            dg.add_node(&init_state, &[("label", &format!("{init_state} (Init)"))]);
        }

        let mut dg_sensors = DotGraph::new("cluster_attacksensors", "subgraph", &[
            ("rank", "sink"),
            ("label", "Attack Sensors"),
            ("fontname", "Helvetica"),
        ]);

        for attack_sensor in &self.data.attack_sensors {
            let node_id = id_name(&attack_sensor.id);

            let mut detail = String::new();

            let valid_states = match attack_sensor.for_state_type.as_str() {
                /*"AttackSensorForStateType.EngagingAndPreAttackOrOutOfReachAndPanic" => &[
                    "RunAway",
                    "Panic",
                    "OutOfReach",
                    "LookingAround",
                    "Engaging",
                    "PreAttack",
                ],
                "AttackSensorForStateType.EngagingOnly" => &["Engaging"],
                "AttackSensorForStateType.PreAttackOnly" => &["PreAttack"],
                "AttackSensorForStateType.WanderingAndIdleOnly" => &["Wandering", "WanderingIdle"],
                "AttackSensorForStateType.AllValid" => &["All"],*/
                // is this useful?
                "AttackSensorForStateType.EngagingAndPreAttackOrOutOfReachAndPanic" => "",
                "AttackSensorForStateType.EngagingOnly" => "Engaging",
                "AttackSensorForStateType.PreAttackOnly" => "PreAttack",
                "AttackSensorForStateType.WanderingAndIdleOnly" => "Wandering[Idle]",
                "AttackSensorForStateType.AllValid" => "",
                _ => todo!("{}", attack_sensor.for_state_type),
            };
            if !valid_states.is_empty() {
                writeln!(
                    &mut detail,
                    "if: state is {}",
                    valid_states,
                    // &enum_value_only(&attack_sensor.for_state_type)
                )?;
            }
            for condition in &attack_sensor.conditions {
                let id = &condition["$id"].as_str().unwrap();
                let id_name = id_name(&id);

                let inverted = condition["FinalResultInverted"].as_bool().unwrap();
                write!(&mut detail, "if{}: ", if inverted { " not" } else { "" },)?;

                if id.ends_with("@PlayerMovePredictCondition") {
                    let detects = [
                        "ParryDetect",
                        "DodgeDetect",
                        "JumpDetect",
                        "InAirDetect",
                        "AttackDetect",
                        "ThirdAttackDetect",
                        "ChargeAttackDetect",
                        "FooDetect",
                        "ArrowDetect",
                    ];
                    for detect in detects {
                        if condition[detect] == true {
                            detail.push_str(&detect.trim_end_matches("Detect"));
                            detail.push_str(" ");
                        }
                    }
                } else {
                    detail.push_str(&id_name);
                }
                detail.push('\n');
            }

            self.add_detail_node(&mut dg_sensors, &node_id, &node_id, &detail.trim_end());

            let linked_states = attack_sensor
                .binding_attacks
                .iter()
                .filter_map(|state| self.data.find_mapping_state_id(state));
            if let Some(table) = table_from_rows(linked_states, 0.0, 2.0) {
                let linked_id = format!("{}_linked", node_id);

                dg_sensors.add_node(&linked_id, &[
                    ("shape", "box"),
                    ("style", "dotted"),
                    ("label", &table),
                    ("margin", "0.05"),
                ]);

                dg_sensors.add_edge(&node_id, &linked_id, &[
                    ("label", "QueueEnd"),
                    ("color", COLOR_END),
                    ("fontcolor", COLOR_END),
                ]);
            }

            let phase_index = 0;
            if let Some(link_move_weight) = attack_sensor.attack_weight_phase_list.get(phase_index)
            {
                self.link_next_move(&mut dg_sensors, &node_id, link_move_weight)
            }
        }
        dg.add_sub_graph(dg_sensors);

        Ok(format!("strict {}", dg.finish()))
    }

    // - must use
    // - initial
    // - weights
    fn link_next_move(
        &self,
        dg: &mut DotGraph,
        base_id: &str,
        next_move_weights: &LinkNextMoveStateWeight,
    ) {
        let must_use = next_move_weights
            .must_use_states
            .iter()
            .filter_map(|state| self.data.get_bgs(&state.reference))
            .map(|bgs| id_name(&bgs.id));

        if let Some(table) = table_from_rows(must_use, 1.0, 4.0) {
            let must_use_id = format!("{}_mustuse", base_id);
            dg.add_node(&must_use_id, &[("shape", "plaintext"), ("label", &table)]);

            dg.add_edge(base_id, &must_use_id, &[
                ("label", "Initial"),
                ("color", COLOR_MUST_USE),
                ("fontcolor", COLOR_MUST_USE),
            ]);
        }

        if !next_move_weights.state_weight_list.is_empty() {
            let (border, padding) = match next_move_weights.is_random {
                true => (0.0, 2.0),
                false => (1.0, 4.0),
            };

            let weighted = next_move_weights.state_weight_list.iter().map(|state| {
                let id = self
                    .data
                    .find_mapping_state_id(&state.state_type)
                    .unwrap_or_else(|| enum_value_only(&state.state_type.0).into());
                format!("{} {id}", state.weight)
            });
            let table = table_from_rows(weighted, border, padding).unwrap();

            let node_attrs: &[_] = match next_move_weights.is_random {
                true => &[("shape", "box"), ("style", "dotted"), ("label", &table)],
                false => &[("shape", "plaintext"), ("label", &table)],
            };

            let next_state_weight_id = format!("{}_mustuse", base_id);
            dg.add_node(&next_state_weight_id, node_attrs);
            dg.add_edge(&base_id, &next_state_weight_id, &[
                ("label", "Weight"),
                ("color", COLOR_WEIGHTS),
                ("fontcolor", COLOR_WEIGHTS),
            ]);
        }
    }

    fn add_detail_node(&self, dg: &mut DotGraph, node_id: &str, label: &str, detail: &str) {
        if detail.is_empty() {
            dg.add_node(node_id, &[("label", node_id)]);
            return;
        }

        let font = format!(
            r#"<FONT point-size="12" color="azure4">{}</FONT>"#,
            detail.replace("\n", "<br/>")
        );

        dg.add_node(&node_id, &[
            ("shape", "plaintext"),
            (
                "label",
                &table_styled(
                    [
                        (label.to_owned(), ""),
                        (font, r#"align="left" balign="left""#),
                    ]
                    .into_iter(),
                    1.0,
                    2.0,
                )
                .unwrap(),
            ),
        ]);
    }
}

fn table_from_rows(
    iter: impl Iterator<Item = impl AsRef<str>>,
    cell_border: f32,
    cell_padding: f32,
) -> Option<String> {
    let mut table = format!(
        r#"RAW:<<TABLE border="0" cellborder="{cell_border}" cellspacing="0" cellpadding="{cell_padding}">"#
    );
    let mut empty = true;
    for row in iter {
        table.push_str(r#"<TR><TD>"#);
        table.push_str(row.as_ref());
        table.push_str("  </TD></TR>");
        empty = false;
    }
    table.push_str("</TABLE>>");
    (!empty).then_some(table)
}
fn table_styled<'a>(
    iter: impl Iterator<Item = (String, &'a str)>,
    cell_border: f32,
    cell_padding: f32,
) -> Option<String> {
    let mut table = format!(
        r#"RAW:<<TABLE border="0" cellborder="{cell_border}" cellspacing="0" cellpadding="{cell_padding}">"#
    );
    let mut empty = true;
    for (row, style) in iter {
        table.push_str("<TR><TD ");
        table.push_str(&style);
        table.push('>');
        table.push_str(row.as_ref());
        table.push_str("  </TD></TR>");
        empty = false;
    }
    table.push_str("</TABLE>>");
    (!empty).then_some(table)
}

fn enum_value_only(x: &str) -> &str {
    match x.split_once('.') {
        None => x,
        Some((_type, value)) => value,
    }
}

pub fn id_name(id: &str) -> Cow<str> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\[[0-9]+]|[0-9]+_)").unwrap());
    const TYPOS: &[(&str, &str)] = &[
        ("Tripple", "Triple"),
        ("Thurst", "Thrust"),
        ("To Close", "Too Close"),
    ];

    let name = id.rsplit_once('/').unwrap().1.split_once('@').unwrap().0;
    let mut name = RE.replace(name.trim(), "");
    for &(typo, replacement) in TYPOS {
        if name.contains(typo) {
            name = name.replace(typo, replacement).into();
        }
    }
    name
}
