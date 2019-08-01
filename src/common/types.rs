use hashbrown::HashMap;
use ordered_float::OrderedFloat;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub(crate) enum Value {
    Int(i64),
    Float(OrderedFloat<f64>),
    String(String),
    Null,
}

pub(crate) type Tuple = Vec<Value>;
pub(crate) type VariableName = String;
pub(crate) type Variables = HashMap<VariableName, Value>;

pub(crate) fn empty_variables() -> Variables {
    HashMap::new()
}