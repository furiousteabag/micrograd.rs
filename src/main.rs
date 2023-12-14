use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::{Add, Deref, Mul};
use std::rc::Rc;

#[derive(Clone)]
enum Operation {
    Add,
    Mul,
}

#[derive(PartialEq, Eq)]
struct Value(Rc<RefCell<ValueInternal>>);

struct ValueInternal {
    data: f32,
    grad: Option<f32>,
    prev: Option<HashSet<Value>>,
    op: Option<Operation>,
    label: Option<String>,
}

impl From<f32> for Value {
    fn from(value: f32) -> Value {
        Value(Rc::new(RefCell::new(ValueInternal {
            data: value,
            grad: None,
            prev: None,
            op: None,
            label: None,
        })))
    }
}

impl From<(f32, Vec<&Value>, Operation)> for Value {
    fn from(value: (f32, Vec<&Value>, Operation)) -> Self {
        let mut prev = HashSet::new();
        for val in value.1 {
            prev.insert(val.clone());
        }
        Value(Rc::new(RefCell::new(ValueInternal {
            data: value.0,
            grad: None,
            prev: Some(prev),
            op: Some(value.2),
            label: None,
        })))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.borrow();
        let grad = value.grad.map_or_else(String::new, |g| g.to_string());
        let label = value.label.clone().unwrap_or_default();
        let op = value.op.clone().map_or_else(String::new, |o| match o {
            Operation::Add => "+".to_string(),
            Operation::Mul => "*".to_string(),
        });

        let prev = if let Some(ref prev_values) = value.prev {
            prev_values
                .iter()
                .map(|v| v.borrow().label.clone().unwrap_or_default())
                .collect::<Vec<_>>()
                .join(" ")
        } else {
            String::new()
        };

        write!(
            f,
            "({} | data: {}{}{})",
            label,
            value.data,
            if grad.is_empty() {
                String::new()
            } else {
                format!(" | grad: {}", grad)
            },
            if prev.is_empty() {
                String::new()
            } else {
                format!(" | prev: {} {}", op, prev)
            }
        )
    }
}

impl Clone for Value {
    fn clone(&self) -> Value {
        Value(Rc::clone(&self.0))
    }
}

// as borrow and borrow_mut automatically dereferencing,
// it let's access fields with simple
// value.borrow().data and value.borrow_mut().data
impl Deref for Value {
    type Target = Rc<RefCell<ValueInternal>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.borrow().hash(state);
    }
}

impl Hash for ValueInternal {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.to_bits().hash(state);
        self.grad.unwrap_or_default().to_bits().hash(state);
    }
}

impl PartialEq for ValueInternal {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Eq for ValueInternal {}

impl Add<Value> for Value {
    type Output = Value;
    fn add(self, rhs: Self) -> Self::Output {
        Value::from((
            self.borrow().data + rhs.borrow().data,
            vec![&self, &rhs],
            Operation::Add,
        ))
    }
}

impl Mul<Value> for Value {
    type Output = Value;
    fn mul(self, rhs: Value) -> Self::Output {
        // self.borrow_mut().grad = rhs.borrow().data;
        // rhs.borrow_mut().grad = self.borrow().data;
        Value::from((
            self.borrow().data * rhs.borrow().data,
            vec![&self, &rhs],
            Operation::Mul,
        ))
    }
}

impl Value {
    fn with_label(self, label: &str) -> Value {
        self.borrow_mut().label = Some(String::from(label));
        self
    }
}

fn main() {
    let a = Value::from(2.0).with_label("a");
    let a_clone = a.clone();

    let b = Value::from(-3.0).with_label("b");
    let b_clone = b.clone();

    let c = Value::from(10.0).with_label("c");
    let c_clone = c.clone();

    let d = Value::from(-2.0).with_label("d");
    let d_clone = d.clone();

    let e = a * b;
    let e_clone = e.clone().with_label("e");

    let g = e + c;
    let g_clone = g.clone().with_label("g");

    let l = g * d;
    let l_clone = l.clone().with_label("l");

    println!("{}", a_clone);
    println!("{}", b_clone);
    println!("{}", c_clone);
    println!("{}", d_clone);
    println!("{}", e_clone);
    println!("{}", g_clone);
    println!("{}", l_clone);
}
