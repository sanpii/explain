#[derive(Debug, serde::Deserialize)]
pub(crate) struct Explain {
    #[serde(rename = "Plan")]
    pub plan: Plan,
    #[serde(rename = "Execution Time", default)]
    pub execution_time: Option<f32>,
    #[serde(rename = "Total Runtime", default)]
    pub total_runtime: Option<f32>,
    #[serde(rename = "Planning Time", default)]
    pub planning_time: Option<f32>,
    #[serde(rename = "Triggers", default)]
    pub triggers: Vec<Trigger>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Plan {
    #[serde(rename = "Actual Loops", default)]
    pub actual_loops: Option<usize>,
    #[serde(rename = "Actual Startup Time", default)]
    pub actual_startup_time: Option<f32>,
    #[serde(rename = "Actual Total Time", default)]
    pub actual_total_time: Option<f32>,
    #[serde(flatten)]
    pub node: Node,
    #[serde(rename = "Output", default)]
    pub output: Vec<String>,
    #[serde(rename = "Parallel Aware", default)]
    pub parallel_aware: bool,
    #[serde(rename = "Parent Relationship")]
    pub parent_relationship: Option<String>,
    #[serde(rename = "Plan Rows")]
    pub rows: u32,
    #[serde(rename = "Plan Width")]
    pub width: u32,
    #[serde(rename = "Plans", default)]
    pub plans: Vec<Plan>,
    #[serde(rename = "Startup Cost")]
    pub startup_cost: f32,
    #[serde(rename = "Subplan Name")]
    pub subplan: Option<String>,
    #[serde(rename = "Total Cost")]
    pub total_cost: f32,
    #[serde(rename = "Workers", default)]
    pub workers: Vec<Worker>,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
#[serde(tag = "Node Type")]
pub(crate) enum Node {
    Aggregate {
        #[serde(rename = "Group Key", default)]
        keys: Vec<String>,
        #[serde(rename = "Partial Mode", default)]
        partial_mode: Option<PartialMode>,
        #[serde(rename = "Strategy")]
        strategy: Strategy,
    },
    Append {},
    BitmapAnd {},
    #[serde(rename = "Bitmap Index Scan")]
    BitmapIndexScan {},
    #[serde(rename = "Bitmap Heap Scan")]
    BitmapHeapScan {},
    BitmapOr {},
    #[serde(rename = "CTE Scan")]
    CteScan {
        #[serde(rename = "CTE Name")]
        name: String,
    },
    CustomScan {},
    #[serde(rename = "Foreign Scan")]
    ForeignScan {},
    #[serde(rename = "Function Scan")]
    FunctionScan {},
    Gather {},
    #[serde(rename = "Gather Merge")]
    GatherMerge {},
    Group {},
    Hash {},
    #[serde(rename = "Hash Join")]
    HashJoin {
        #[serde(rename = "Join Type")]
        join_type: JoinType,
        #[serde(rename = "Inner Unique", default)]
        inner_unique: bool,
        #[serde(rename = "Hash Cond")]
        hash_cond: String,
    },
    #[serde(rename = "Index Scan")]
    IndexScan {},
    #[serde(rename = "Index Only Scan")]
    IndexOnlyScan {
        #[serde(rename = "Scan Direction")]
        scan_direction: String,
        #[serde(rename = "Index Name")]
        index_name: String,
        #[serde(flatten)]
        relation: Relation,
    },
    Limit {},
    LockRows {},
    Materialize {},
    #[serde(rename = "Merge Append")]
    MergeAppend {},
    #[serde(rename = "Merge Join")]
    MergeJoin {
        #[serde(rename = "Join Type")]
        join_type: JoinType,
    },
    ModifyTable {
        #[serde(rename = "Operation")]
        operation: Operation,
        #[serde(flatten)]
        relation: Relation,
    },
    #[serde(rename = "NamedTuplestoreScan")]
    NamedTuplestoreScan {},
    #[serde(rename = "Nested Loop")]
    NestedLoop {
        #[serde(rename = "Join Type")]
        join_type: JoinType,
    },
    #[serde(rename = "ProjectSet")]
    ProjectSet {},
    #[serde(rename = "Recursive Union")]
    RecursiveUnion {},
    #[serde(rename = "Result")]
    Result {},
    #[serde(rename = "Sample Scan")]
    SampleScan {},
    #[serde(rename = "Seq Scan")]
    SeqScan {
        #[serde(flatten)]
        relation: Relation,
    },
    SetOp {
        #[serde(rename = "Strategy")]
        strategy: Strategy,
    },
    Sort {
        #[serde(rename = "Sort Key")]
        keys: Vec<String>,
    },
    #[serde(rename = "Subquery Scan")]
    SubqueryScan {},
    #[serde(rename = "Table Function Scan")]
    TableFunctionScan {},
    #[serde(rename = "Tid Scan")]
    TidScan {},
    Unique {},
    #[serde(rename = "Value Scan")]
    ValueScan {},
    WindowAgg {},
    #[serde(rename = "WorkTable Scan")]
    WorkTableScan {},
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Aggregate { .. } => "Aggregate",
            Self::Append { .. } => "Append",
            Self::BitmapAnd { .. } => "BitmapAnd",
            Self::BitmapHeapScan { .. } => "Bitmap Heap Scan",
            Self::BitmapIndexScan { .. } => "Bitmap Index Scan",
            Self::BitmapOr { .. } => "BitmapOr",
            Self::CteScan { .. } => "CteScan",
            Self::CustomScan { .. } => "CustomScan",
            Self::ForeignScan { .. } => "Foreign Scan",
            Self::FunctionScan { .. } => "FunctionScan",
            Self::Gather { .. } => "Gather",
            Self::GatherMerge { .. } => "Gather Merge",
            Self::Group { .. } => "Group",
            Self::Hash { .. } => "Hash",
            Self::HashJoin { .. } => "Hash Join",
            Self::IndexOnlyScan { .. } => "Index Only Scan",
            Self::IndexScan { .. } => "Index Scan",
            Self::Limit { .. } => "Limit",
            Self::LockRows { .. } => "LockRows",
            Self::Materialize { .. } => "Materialize",
            Self::MergeAppend { .. } => "Merge Append",
            Self::MergeJoin { .. } => "Merge Join",
            Self::ModifyTable { .. } => "ModifyTable",
            Self::NamedTuplestoreScan { .. } => "Named Tuplestore Scan",
            Self::NestedLoop { .. } => "Nested Loop",
            Self::ProjectSet { .. } => "ProjectSet",
            Self::RecursiveUnion { .. } => "Recursive Union",
            Self::Result { .. } => "Result",
            Self::SampleScan { .. } => "Sample Scan",
            Self::SeqScan { .. } => "Seq Scan",
            Self::SetOp { .. } => "SetOp",
            Self::Sort { .. } => "Sort",
            Self::SubqueryScan { .. } => "Subquery Scan",
            Self::TableFunctionScan { .. } => "Table Funciton Scan",
            Self::TidScan { .. } => "Tid Scan",
            Self::Unique { .. } => "Unique",
            Self::ValueScan { .. } => "Value Scan",
            Self::WindowAgg { .. } => "WindowAgg",
            Self::WorkTableScan { .. } => "WorkTable Scan",
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub(crate) enum Operation {
    Delete,
    Insert,
    Select,
    Update,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub(crate) enum Strategy {
    Hashed,
    Mixed,
    Plain,
    Sorted,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub(crate) enum PartialMode {
    Finalize,
    Partial,
    Simple,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub(crate) enum JoinType {
    Anti,
    Full,
    Inner,
    Left,
    Right,
    Semi,
}

impl std::fmt::Display for JoinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Anti => "anti",
            Self::Full => "full",
            Self::Inner => "inner",
            Self::Left => "left",
            Self::Right => "right",
            Self::Semi => "semi",
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub(crate) struct Relation {
    #[serde(rename = "Relation Name")]
    name: String,
    #[serde(rename = "Alias")]
    alias: String,
    #[serde(rename = "Schema", default = "default_schema")]
    schema: String,
}

fn default_schema() -> String {
    "public".to_string()
}

impl std::fmt::Display for Relation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}({})", self.schema, self.name, self.alias)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Trigger {
    #[serde(rename = "Trigger Name")]
    name: String,
    #[serde(rename = "Relation")]
    relation: String,
    #[serde(rename = "Time")]
    time: f32,
    #[serde(rename = "Calls")]
    calls: u32,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Worker {
    #[serde(rename = "Worker Number")]
    number: usize,
    #[serde(rename = "Actual Startup Time")]
    actual_startup_time: f32,
    #[serde(rename = "Actual Total Time")]
    actual_total_time: f32,
    #[serde(rename = "Actual Rows")]
    actual_rows: usize,
    #[serde(rename = "Actual Loops")]
    actual_loops: usize,
}
