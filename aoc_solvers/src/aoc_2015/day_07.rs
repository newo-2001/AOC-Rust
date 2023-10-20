use std::cell::RefCell;

use ahash::{HashMap, HashMapExt};
use aoc_lib::{functional::swap, parsing::{ParseError, TextParser, Parsable, TextParserResult, Map2, lines}, between};
use aoc_runner_api::SolverResult;
use nom::{character::complete::{alpha1, u32, char}, sequence::{preceded, separated_pair}, bytes::complete::tag, Parser, branch::alt};

#[derive(Clone, Hash, PartialEq, Eq)]
struct Wire<'a>(&'a str);

impl<'a> Parsable<'a> for Wire<'a> {
    fn parse(input: &'a str) -> TextParserResult<'_, Self> {
        alpha1.map(Wire).parse(input)
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
enum Value<'a> { 
    Wire(Wire<'a>),
    Literal(u32)
}

impl<'a> Parsable<'a> for Value<'a> {
    fn parse(input: &'a str) -> TextParserResult<'_, Self> {
        Parser::or(
            u32.map(Value::Literal),
            Wire::parse.map(Value::Wire)
        ).parse(input)
    }
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

impl<'a> Parsable<'a> for Expression<'a> {
    fn parse(input: &'a str) -> TextParserResult<'_, Self> {
        fn binary_operator<'a, M, X, Y>(name: &'a str, mapper: M) -> impl TextParser<'a, Expression>
            where M: Fn(X, Y) -> Expression<'a>,
                X: Parsable<'a>, Y: Parsable<'a>
        {
            separated_pair(
                X::parse,
                between!(char(' '), tag(name)),
                Y::parse
            ).map2(mapper)
        }

        alt((
            binary_operator("AND", Expression::And),
            binary_operator("OR", Expression::Or),
            binary_operator("LSHIFT", Expression::LeftShift),
            binary_operator("RSHIFT", Expression::RightShift),
            preceded(tag("NOT "), Value::parse).map(Expression::Not),
            Value::parse.map(Expression::Constant)
        )).parse(input)
    }
}

struct ExpressionTree<'a> {
    nodes: HashMap<Wire<'a>, Expression<'a>>,
    cache: RefCell<HashMap<Wire<'a>, u32>>
}

impl<'a> FromIterator<(Wire<'a>, Expression<'a>)> for ExpressionTree<'a> {
    fn from_iter<T: IntoIterator<Item = (Wire<'a>, Expression<'a>)>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl<'a> ExpressionTree<'a> {
    fn new(nodes: HashMap<Wire<'a>, Expression<'a>>) -> ExpressionTree<'a> {
        ExpressionTree {
            cache: RefCell::new(HashMap::<Wire, u32>::new()), nodes
        }
    }
    
    fn parse(input: &'a str) -> Result<ExpressionTree<'a>, ParseError<'a>> {
        let expression_tree = lines(
            separated_pair(
                Expression::parse,
                tag(" -> "),
                Wire::parse
            ).map(swap)
        ).run(input)?
            .into_iter()
            .collect();

        Ok(expression_tree)
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