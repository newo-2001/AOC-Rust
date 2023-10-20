use std::cell::RefCell;

use ahash::{HashMap, HashMapExt};
use aoc_lib::{functional::swap, parsing::{ParseError, parse_lines, TextParser}};
use aoc_runner_api::SolverResult;
use nom::{character::complete::{alpha1, u32, u8}, sequence::{terminated, preceded}, bytes::complete::tag, Parser, branch::alt};

#[derive(Clone, Hash, PartialEq, Eq)]
struct Wire<'a>(&'a str);

#[derive(Clone, Hash, PartialEq, Eq)]
enum Value<'a> {
    Wire(Wire<'a>),
    Literal(u32)
}

#[derive(Clone, Hash, PartialEq, Eq)]
enum Expression<'a> {
    Constant(Value<'a>),
    And(Value<'a>, Value<'a>),
    Or(Value<'a>, Value<'a>),
    Not(Value<'a>),
    LeftShift(Value<'a>, u8),
    RightShift(Value<'a>, u8)
}

struct ExpressionTree<'a> {
    nodes: HashMap<Wire<'a>, Expression<'a>>,
    cache: RefCell<HashMap<Wire<'a>, u32>>
}

impl<'a> ExpressionTree<'a> {
    fn new(nodes: HashMap<Wire<'a>, Expression<'a>>) -> ExpressionTree<'a> {
        ExpressionTree {
            cache: RefCell::new(HashMap::<Wire, u32>::new()), nodes
        }
    }
    
    fn parse(input: &'a str) -> Result<ExpressionTree<'a>, ParseError<'a>> {
        let nodes = parse_lines(parse_wire, input)?
            .into_iter()
            .collect::<HashMap<Wire, Expression>>();

        Ok(ExpressionTree::new(nodes))
    }

    fn reset(&mut self) {
        self.cache.replace(HashMap::new());
    }

    fn evaluate_value(&self, value: &Value<'a>) -> u32 {
        match value {
            Value::Literal(value) => *value,
            Value::Wire(wire) => self.evaluate_wire(wire)
        }
    }

    // TODO: using stack recursion is possibly dangerous
    // This can be solved by moving the stack to the heap
    fn evaluate_wire(&self, wire: &Wire<'a>) -> u32 {
        if let Some(value) = self.cache.borrow().get(wire) { return *value; }
        let expression = self.nodes.get(wire).unwrap();
        let value = match expression {
            Expression::Constant(value) => self.evaluate_value(value),
            Expression::And(left, right) => self.evaluate_value(left) & self.evaluate_value(right),
            Expression::Or(left, right) => self.evaluate_value(left) | self.evaluate_value(right),
            Expression::Not(value) => !self.evaluate_value(value),
            Expression::LeftShift(value, amount) => self.evaluate_value(value) << amount,
            Expression::RightShift(value, amount) => self.evaluate_value(value) >> amount
        };

        self.cache.borrow_mut().insert(wire.clone(), value);
        value
    }
}

fn parse_wire(input: &str) -> Result<(Wire, Expression), ParseError> {
    let wire = || alpha1.map(Wire);
    let literal = || u32.map(Value::Literal);
    let value = || wire().map(Value::Wire).or(literal());

    let constant = value().map(Expression::Constant);
    let and = terminated(value(), tag(" AND ")).and(value()).map(|(left, right)| Expression::And(left, right));
    let or = terminated(value(), tag(" OR ")).and(value()).map(|(left, right)| Expression::Or(left, right));
    let lshift = terminated(value(), tag(" LSHIFT ")).and(u8).map(|(value, amount)| Expression::LeftShift(value, amount));
    let rshift = terminated(value(), tag(" RSHIFT ")).and(u8).map(|(value, amount)| Expression::RightShift(value, amount));
    let not = preceded(tag("NOT "), value()).map(Expression::Not);

    let expression = alt((and, or, lshift, rshift, not, constant));    
    terminated(expression, tag(" -> ")).and(wire())
        .map(swap).run(input)
}

const WIRE_A: Wire<'static> = Wire("a");

pub fn solve_part_1(input: &str) -> SolverResult {
    let tree = ExpressionTree::parse(input)?;
    let value = tree.evaluate_wire(&WIRE_A);
    Ok(Box::new(value))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut tree = ExpressionTree::parse(input)?;
    let value = tree.evaluate_wire(&WIRE_A);

    tree.reset();
    tree.nodes.insert(Wire("b"), Expression::Constant(Value::Literal(value)));
    let value = tree.evaluate_wire(&WIRE_A);

    Ok(Box::new(value))
}