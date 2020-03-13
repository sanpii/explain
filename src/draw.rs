pub(crate) fn graph(explain: &crate::Explain) -> String {
    let (_, s) = self::plan(&explain.plan, None, &explain.plan);

    format!(
        r#"
graph "" {{
    node[shape=plain,style=rounded];

    {}
}}
"#,
        s
    )
}

fn plans(root: &crate::Plan, root_id: Option<u32>, nodes: &[crate::Plan]) -> String {
    let mut plans = String::new();

    for node in nodes {
        let (_, plan) = &plan(root, root_id, &node);
        plans.push_str(plan);
    }

    plans
}

lazy_static::lazy_static! {
    static ref ID: std::sync::Mutex<u32> = std::sync::Mutex::new(0);
}

fn plan(root: &crate::Plan, root_id: Option<u32>, plan: &crate::Plan) -> (u32, String) {
    let id = *ID.lock().unwrap();
    *ID.lock().unwrap() += 1;

    let children = plans(root, Some(id), &plan.plans);
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

    let mut node = format!(
        r#"
node{id}[
    label=<
    <table cellborder="0" cellspacing="5">
        <tr><td align="left"><b>{ty}</b></td>{time}</tr>
        <tr><td colspan="2" align="left">{info}</td></tr>
        <tr><td colspan="2" border="1" bgcolor="{color};{percent:.2}:white">Score: {score}</td></tr>
        <tr><td colspan="2" align="left">Rows: {row}</td></tr>
    </table>
>
];

{children}"#,
        id = id,
        ty = plan.node,
        info = info,
        score = plan.total_cost,
        percent = percent,
        color = color,
        row = plan.rows,
        time = time,
        children = children
    );

    if let Some(root_id) = root_id {
        node.push_str(&format!("node{} -- node{}\n", root_id, id));
    }

    (id, node)
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
        crate::Node::Aggregate { keys, .. } => format!("by {}", keys.join(", ")),
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
