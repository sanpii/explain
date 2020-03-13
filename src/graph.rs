type Nd = usize;
type Ed<'a> = &'a (usize, usize);

struct Graph {
    nodes: Vec<Node>,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

struct Node {
    ty: String,
    info: String,
    percent: f32,
    color: String,
    time: String,
    total_cost: f32,
    rows: u32,
}

impl Node {
    fn from(root: &crate::Plan, plan: &crate::Plan) -> Self {
        let info = info(&plan);
        let percent = (plan.total_cost / root.total_cost).min(0.99).max(0.1);
        let color = color(percent);
        let time = if let Some(time) = time(&plan) {
            let time_percent = (time / root.actual_total_time.unwrap() * 100.)
                .round()
                .trunc();

            if time < 1. {
                format!("<td>&lt; 1 ms | {} %</td>", time_percent)
            } else {
                format!("<td>{} ms | {} %</td>", time, time_percent)
            }
        } else {
            String::new()
        };

        Self {
            ty: plan.node.to_string(),
            info,
            percent,
            color,
            time,
            total_cost: plan.total_cost,
            rows: plan.rows,
        }
    }
}

pub(crate) fn dot(explain: &crate::Explain) -> String {
    let mut graph = Graph::new();

    self::plan(&mut graph, &explain.plan, None, &explain.plan);

    let mut output = Vec::new();

    dot::render(&graph, &mut output).unwrap();

    std::str::from_utf8(&output).unwrap().to_string()
}

lazy_static::lazy_static! {
    static ref ID: std::sync::Mutex<usize> = std::sync::Mutex::new(0);
}

fn plan(graph: &mut Graph, root: &crate::Plan, root_id: Option<usize>, plan: &crate::Plan) {
    let id = *ID.lock().unwrap();
    *ID.lock().unwrap() += 1;

    graph.nodes.push(Node::from(root, plan));
    if let Some(root_id) = root_id {
        graph.edges.push((root_id, id));
    }

    for child in &plan.plans {
        self::plan(graph, root, Some(id), child);
    }
}

impl<'a> dot::Labeller<'a, Nd, Ed<'a>> for Graph {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("explain").unwrap()
    }

    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        dot::Id::new(format!("node{}", n)).unwrap()
    }

    fn node_label<'b>(&'b self, n: &Nd) -> dot::LabelText<'b> {
        let node = &self.nodes[*n];

        let label = format!(
            r#"<table cellborder="0" cellspacing="5">
    <tr><td align="left"><b>{ty}</b></td>{time}</tr>
    <tr><td colspan="2" align="left">{info}</td></tr>
    <tr><td colspan="2" border="1" bgcolor="{color};{percent:.2}:white">Score: {score}</td></tr>
    <tr><td colspan="2" align="left">Rows: {row}</td></tr>
</table>"#,
            ty = node.ty,
            info = node.info,
            score = node.total_cost,
            percent = node.percent,
            color = node.color,
            row = node.rows,
            time = node.time,
        );

        dot::LabelText::HtmlStr(label.into())
    }

    fn node_shape(&'a self, _: &Nd) -> Option<dot::LabelText<'a>> {
        Some(dot::LabelText::LabelStr("plain".into()))
    }

    fn node_style(&'a self, _: &Nd) -> dot::Style {
        dot::Style::Rounded
    }

    fn kind(&self) -> dot::Kind {
        dot::Kind::Graph
    }
}

impl<'a> dot::GraphWalk<'a, Nd, Ed<'a>> for Graph {
    fn nodes(&self) -> dot::Nodes<'a, Nd> {
        (0..self.nodes.len()).collect()
    }

    fn edges(&'a self) -> dot::Edges<'a, Ed<'a>> {
        self.edges.iter().collect()
    }

    fn source(&self, e: &Ed) -> Nd {
        e.0
    }

    fn target(&self, e: &Ed) -> Nd {
        e.1
    }
}

fn color(percent: f32) -> String {
    let hue = (100. - percent * 100.) * 1.2 / 360.;
    let (red, green, blue) = hsl_to_rgb(hue, 0.9, 0.4);

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
            hue2rgb(p, q, hue + 1. / 3.),
            hue2rgb(p, q, hue),
            hue2rgb(p, q, hue - 1. / 3.),
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

fn info(plan: &crate::Plan) -> String {
    match &plan.node {
        crate::Node::Aggregate { keys, .. } => if keys.len() > 0 {
            format!("by {}", keys.join(", "))
        } else {
            String::new()
        },
        crate::Node::HashJoin {
            join_type,
            hash_cond,
            ..
        } => format!("{} join on {}", join_type, hash_cond),
        crate::Node::Sort { keys, .. } => format!("by {}", keys.join(", ")),
        crate::Node::SeqScan { relation, .. } => format!("on {}", relation),
        _ => String::new(),
    }
}

fn time(plan: &crate::Plan) -> Option<f32> {
    // @TODO soustraire le actual_total_time des enfant direct

    plan.actual_total_time
}
