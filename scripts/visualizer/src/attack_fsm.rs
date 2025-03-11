use crate::dot::Table;
use crate::json::{AnimationEvent, State};
use crate::{
    dot::DotGraph,
    json::{Data, LinkNextMoveStateWeight},
};
use anyhow::Result;
use regex::Regex;
use std::{borrow::Cow, fmt::Write as _, sync::LazyLock};

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

        dg.add_sub_graph(self.attack_sensors()?);

        self.add_attack_states(&mut dg)?;

        if let Some(init_state) = self.data.find_mapping_state_id(&self.data.init_state) {
            dg.add_node(&init_state, &[("label", &format!("{init_state} (Init)"))]);
        }

        Ok(format!("strict {}", dg.finish()))
    }

    fn add_linked_states(
        &self,
        dg: &mut DotGraph,
        base_id: &str,
        linked_states: &[State],
    ) -> Result<()> {
        let mut table = default_table(0.0, 2.0);

        linked_states
            .iter()
            .filter_map(|state| self.data.find_mapping_state_id(state))
            .for_each(|state| table.add_row(&state, &[]));

        if let Some(table) = table.finish_nonempty() {
            let linked_id = format!("{}_linked", base_id);

            dg.add_node(&linked_id, &[
                ("shape", "box"),
                ("style", "dotted"),
                ("label", &table),
                ("margin", "0.05"),
            ]);

            dg.add_edge(&base_id, &linked_id, &[
                ("label", "QueueEnd"),
                ("color", COLOR_END),
                ("fontcolor", COLOR_END),
            ]);
        }
        Ok(())
    }

    fn animation_clip_text(&self, clip: Option<&[AnimationEvent]>) -> String {
        let clip = clip.as_deref().unwrap_or_default();
        clip.iter()
            .filter_map(|e| {
                Some(format!(
                    "{:.2}s: {}",
                    e.time,
                    enum_value_only(e.event.as_deref()?)
                ))
            })
            .collect::<Vec<_>>()
            .join("<br/>")
    }

    fn add_attack_states(&self, dg: &mut DotGraph) -> Result<()> {
        for bgs in self.data.boss_general_states.values() {
            let node_id = id_name(&bgs.id);

            let mut table =
                self.detail_table(&node_id, &self.animation_clip_text(bgs.clip.as_deref()));

            let has_groups = !bgs.grouping_nodes.is_empty();
            if has_groups {
                for (i, group) in bgs.grouping_nodes.iter().enumerate() {
                    let port = format!("group{}", i.to_string());
                    table.add_row(&format!("Group {i}"), &[("PORT", &port)]);

                    let phase_index = 0;
                    let next_move_weights = &group.queue.link_next_move_state_weights[phase_index];
                    self.link_next_move(dg, &node_id, Some(&port), next_move_weights);
                    // dg.add_edge_with_ports(&node_id, Some(&port), &port, None, &[]);
                }
            } else {
            }
            if let Some(exit_state) = self.data.find_mapping_state_id(&bgs.exit_state) {
                dg.add_edge(&node_id, &exit_state, &[
                    ("color", COLOR_EXIT),
                    ("fontcolor", COLOR_EXIT),
                ]);
            }
            self.add_linked_states(dg, &node_id, &bgs.linked_state_types)?;

            let phase_index = 0;

            if let Some(next_move_weights) = bgs.link_next_move_state_weights.get(phase_index) {
                self.link_next_move(dg, &node_id, None, next_move_weights);
            }

            dg.add_node(&node_id, &[
                ("shape", "plaintext"),
                ("label", &table.finish()),
            ]);
        }

        Ok(())
    }

    /* let mut other = DotGraph::new("other", "subgraph", &[("rank", "sink")]);
    for state in data.other_states.values() {
        let name = id_name(&state.id);
        other.add_node(&id_name(&state.id), &[("label", &name)]);
        dg.add_edge(
            &data.boss_general_states.iter().last().unwrap().1.node_id(),
            &id_name(&state.id),
            &[("style", "invis")],
        );
    }
    dg.add_sub_graph(other); */

    fn attack_sensors(&self) -> Result<DotGraph> {
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

            let table = self.detail_table(&node_id, &detail.trim_end());
            dg_sensors.add_node(&node_id, &[
                ("shape", "plaintext"),
                ("label", &table.finish()),
            ]);

            let mut table = default_table(0.0, 2.0);
            attack_sensor
                .binding_attacks
                .iter()
                .filter_map(|state| self.data.find_mapping_state_id(state))
                .for_each(|state| table.add_row(&state, &[]));

            if let Some(table) = table.finish_nonempty() {
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
                self.link_next_move(&mut dg_sensors, &node_id, None, link_move_weight)
            }
        }

        Ok(dg_sensors)
    }

    // - must use
    // - initial
    // - weights
    fn link_next_move(
        &self,
        dg: &mut DotGraph,
        base_id: &str,
        port: Option<&str>,
        next_move_weights: &LinkNextMoveStateWeight,
    ) {
        let mut table = default_table(1.0, 4.0);
        next_move_weights
            .must_use_states
            .iter()
            .filter_map(|state| self.data.get_bgs(&state.reference))
            .map(|bgs| id_name(&bgs.id))
            .for_each(|state| table.add_row(&state, &[]));

        if let Some(table) = table.finish_nonempty() {
            let must_use_id = format!("{}_mustuse", base_id);
            dg.add_node(&must_use_id, &[("shape", "plaintext"), ("label", &table)]);

            dg.add_edge_with_ports(base_id, port, &must_use_id, None, &[
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

            let mut table = default_table(border, padding);
            for state in &next_move_weights.state_weight_list {
                let id = self
                    .data
                    .find_mapping_state_id(&state.state_type)
                    .unwrap_or_else(|| enum_value_only(&state.state_type.0).into());
                table.add_row(&format!("{} {id}", state.weight), &[]);
            }
            let table = table.finish();

            let node_attrs: &[_] = match next_move_weights.is_random {
                true => &[("shape", "box"), ("style", "dotted"), ("label", &table)],
                false => &[("shape", "plaintext"), ("label", &table)],
            };

            let next_state_weight_id = format!("{}{}_weight", port.unwrap_or_default(), base_id);
            dg.add_node(&next_state_weight_id, node_attrs);
            dg.add_edge_with_ports(&base_id, port, &next_state_weight_id, None, &[
                ("label", "Weight"),
                ("color", COLOR_WEIGHTS),
                ("fontcolor", COLOR_WEIGHTS),
            ]);
        }
    }

    fn detail_table(&self, label: &str, detail: &str) -> Table {
        let mut table = default_table(1.0, 2.0);
        table.add_row(&label, &[]);
        if !detail.is_empty() {
            let font = format!(
                r#"<FONT point-size="12" color="azure4">{}</FONT>"#,
                detail.replace("\n", "<br/>")
            );
            table.add_row(&font, &[("align", "left"), ("balign", "left")]);
        }
        table
    }
}

fn default_table(cell_border: f32, cell_padding: f32) -> Table {
    Table::new(&[
        ("border", "0"),
        ("cellspacing", "0"),
        ("cellborder", &cell_border.to_string()),
        ("cellpadding", &cell_padding.to_string()),
    ])
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
