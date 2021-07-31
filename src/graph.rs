pub(crate) fn dot(explain: &crate::Explain) -> String {
    Graph::from(explain).render()
}

type Su<'a> = String;
type Nd = usize;
type Ed<'a> = &'a (usize, usize);

#[derive(Debug, Default)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<(usize, usize)>,
    current_id: usize,
    max_cost: f32,
    execution_time: Option<f32>,
}

impl Graph {
    fn new() -> Self {
        Self::default()
    }

    fn from(explain: &crate::Explain) -> Self {
        let mut graph = Self::new();

        graph.execution_time = explain
            .execution_time
            .or(explain.total_runtime)
            .or(explain.plan.actual_total_time);
        graph.plan(explain, None, &explain.plan);

        graph
    }

    fn plan(&mut self, explain: &crate::Explain, root: Option<&Node>, plan: &crate::Plan) {
        let id = self.current_id;
        self.current_id += 1;

        let mut node = Node::from(id, plan);
        if node.subplan.is_none() {
            node.subplan = root.map(|x| x.subplan.clone()).flatten();
        }

        if node.cost > self.max_cost {
            self.max_cost = node.cost;
        }
        self.nodes.push(node.clone());

        if let Some(root) = root {
            self.edges.push((root.id, id));
        }

        for child in &plan.plans {
            self.plan(explain, Some(&node), child);
        }
    }

    fn render(&self) -> String {
        let mut output = Vec::new();

        dot::render(self, &mut output).unwrap();

        std::str::from_utf8(&output).unwrap().to_string()
    }

    fn node(&self, n: Nd) -> Option<&Node> {
        self.nodes.get(n)
    }

    fn duration_color(percent: f32) -> &'static str {
        if percent > 90. {
            "#880000"
        } else if percent > 40. {
            "#ee8800"
        } else if percent > 10. {
            "#fddb61"
        } else {
            "white"
        }
    }

    fn color(percent: f32) -> String {
        let hue = (100. - percent * 100.) * 1.2 / 360.;
        let (red, green, blue) = Self::hsl_to_rgb(hue, 0.9, 0.4);

        format!("#{:02x}{:02x}{:02x}", red, green, blue)
    }

    fn hsl_to_rgb(hue: f32, saturation: f32, lightness: f32) -> (u8, u8, u8) {
        let (red, green, blue) = if saturation != 0. {
            let q = if lightness < 0.5 {
                lightness * (1. + saturation)
            } else {
                lightness + saturation - lightness * saturation
            };
            let p = 2. * lightness - q;

            (
                Self::hue2rgb(p, q, hue + 1. / 3.),
                Self::hue2rgb(p, q, hue),
                Self::hue2rgb(p, q, hue - 1. / 3.),
            )
        } else {
            (lightness, lightness, lightness)
        };

        (
            (red * 255.) as u8,
            (green * 255.) as u8,
            (blue * 255.) as u8,
        )
    }

    fn hue2rgb(p: f32, q: f32, t: f32) -> f32 {
        let mut t = t;

        if t < 0. {
            t += 1.;
        }
        if t > 1. {
            t -= 1.;
        }
        if t < 1. / 6. {
            return p + (q - p) * 6. * t;
        }
        if t < 1. / 2. {
            return q;
        }
        if t < 2. / 3. {
            return p + (q - p) * (2. / 3. - t) * 6.;
        }

        p
    }
}

#[derive(Clone, Debug)]
struct Node {
    id: usize,
    cost: f32,
    executed: bool,
    info: String,
    n_workers: usize,
    rows: u32,
    time: Option<f32>,
    ty: String,
    subplan: Option<String>,
}

impl Node {
    fn from(id: usize, plan: &crate::Plan) -> Self {
        Self {
            id,
            cost: Self::cost(plan),
            executed: plan.actual_loops != Some(0),
            info: Self::info(plan),
            n_workers: plan.workers.len(),
            rows: plan.rows,
            time: Self::time(plan),
            ty: plan.node.to_string(),
            subplan: plan.subplan.clone(),
        }
    }

    fn info(plan: &crate::Plan) -> String {
        let info = match &plan.node {
            crate::Node::Aggregate { keys, .. } => {
                if keys.is_empty() {
                    String::new()
                } else {
                    format!("by {}", keys.join(", "))
                }
            }
            crate::Node::HashJoin {
                join_type,
                hash_cond,
                ..
            } => format!("{} join on {}", join_type, hash_cond),
            crate::Node::Sort { keys, .. } => format!("by {}", keys.join(", ")),
            crate::Node::SeqScan { relation, .. } => format!("on {}", relation),
            _ => String::new(),
        };

        if info.len() > 80 {
            format!("{}…", info.split_at(80).0)
        } else {
            info
        }
    }

    fn time(plan: &crate::Plan) -> Option<f32> {
        if let Some(mut time) = plan.actual_total_time {
            if plan.workers.is_empty() {
                time *= plan.actual_loops.unwrap_or(1) as f32;
            }

            for child in &plan.plans {
                if child.parent_relationship != Some("InitPlan".to_string()) {
                    time -= if child.workers.is_empty() {
                        child.actual_total_time.unwrap_or_default()
                            * child.actual_loops.unwrap_or(1) as f32
                    } else {
                        child.actual_total_time.unwrap_or_default()
                    }
                }
            }

            Some(time)
        } else {
            None
        }
    }

    fn cost(plan: &crate::Plan) -> f32 {
        let mut cost = plan.total_cost;

        for child in &plan.plans {
            if child.parent_relationship != Some("InitPlan".to_string()) {
                cost -= child.total_cost;
            }
        }

        cost.max(0.)
    }
}

impl<'a> dot::Labeller<'a, Nd, Ed<'a>, Su<'a>> for Graph {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("explain").unwrap()
    }

    fn subgraph_id(&'a self, s: &Su<'a>) -> Option<dot::Id<'a>> {
        dot::Id::new(format!("cluster_{}", s.trim_start_matches("CTE "))).ok()
    }

    fn subgraph_label(&'a self, s: &Su<'a>) -> dot::LabelText<'a> {
        let label = format!("<b>{}</b>", s);

        dot::LabelText::HtmlStr(label.into())
    }

    fn subgraph_style(&'a self, _: &Su<'a>) -> dot::Style {
        dot::Style::Filled
    }

    fn subgraph_color(&'a self, _: &Su<'a>) -> Option<dot::LabelText<'a>> {
        Some(dot::LabelText::LabelStr("lightgrey".into()))
    }

    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        dot::Id::new(format!("node{}", n)).unwrap()
    }

    fn node_label<'b>(&'b self, n: &Nd) -> dot::LabelText<'b> {
        let node = self.node(*n).unwrap();
        let percent = node.cost / self.max_cost;
        let color = Self::color(percent);

        let bgcolor = if percent < 0.1 {
            String::new()
        } else if percent > 0.99 {
            format!("bgcolor=\"{}\"", color)
        } else {
            format!("bgcolor=\"{};{:.2}:white\"", color, percent)
        };

        let time = if let Some(time) = node.time {
            let time_percent = (time / self.execution_time.unwrap() * 100.).round().trunc();

            if !node.executed {
                "<td><font color=\"gray\">Never executed</font></td>".to_string()
            } else if time < 1. {
                format!("<td>&lt; 1 ms | {} %</td>", time_percent)
            } else {
                let color = Self::duration_color(time_percent);

                format!(
                    "<td bgcolor=\"{}\">{:.2} ms | {} %</td>",
                    color, time, time_percent
                )
            }
        } else {
            String::new()
        };

        let mut label = r#"<table border="0" cellborder="0" cellspacing="5">"#.to_string();
        label.push_str(&format!(
            r#"<tr><td align="left"><b>{}</b></td>{}</tr>"#,
            node.ty, time
        ));
        label.push_str(&format!(
            r#"<tr><td colspan="2" align="left">{}</td></tr>"#,
            node.info
        ));
        if node.n_workers > 0 {
            label.push_str(&format!(
                r#"<tr><td colspan="2" align="left">Workers: {}</td></tr>"#,
                node.n_workers
            ));
        }

        label.push_str(&format!(
            r#"<tr><td colspan="2" border="1" {}>Cost: {:.02}</td></tr>"#,
            bgcolor, node.cost
        ));
        label.push_str(&format!(
            r#"<tr><td colspan="2" align="left">Rows: {}</td></tr>"#,
            node.rows
        ));
        label.push_str("</table>");

        dot::LabelText::HtmlStr(label.into())
    }

    fn node_shape(&'a self, n: &Nd) -> Option<dot::LabelText<'a>> {
        let node = match self.node(*n) {
            Some(node) => node,
            None => return None,
        };

        let shape = if node.n_workers > 0 { "folder" } else { "box" };

        Some(dot::LabelText::LabelStr(shape.into()))
    }

    fn node_style(&'a self, _: &Nd) -> dot::Style {
        dot::Style::Rounded
    }

    fn node_color(&'a self, n: &Nd) -> Option<dot::LabelText<'a>> {
        let node = match self.node(*n) {
            Some(node) => node,
            None => return None,
        };

        if node.executed {
            None
        } else {
            Some(dot::LabelText::LabelStr("gray".into()))
        }
    }

    fn kind(&self) -> dot::Kind {
        dot::Kind::Graph
    }
}

impl<'a> dot::GraphWalk<'a, Nd, Ed<'a>, Su<'a>> for Graph {
    fn subgraphs(&'a self) -> dot::Subgraphs<'a, Su<'a>> {
        let mut s: Vec<String> = self
            .nodes
            .iter()
            .filter_map(|n| n.subplan.clone())
            .collect();

        s.dedup();

        s.into()
    }

    fn subgraph_nodes(&'a self, s: &Su<'a>) -> dot::Nodes<'a, Nd> {
        self.nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.subplan == Some(s.to_string()))
            .map(|(k, _)| k)
            .collect()
    }

    fn nodes(&self) -> dot::Nodes<'a, Nd> {
        (0..self.nodes.len()).collect()
    }

    fn edges(&'a self) -> dot::Edges<'a, Ed<'a>> {
        self.edges.iter().collect()
    }

    fn source(&self, e: &Ed<'_>) -> Nd {
        e.0
    }

    fn target(&self, e: &Ed<'_>) -> Nd {
        e.1
    }
}
