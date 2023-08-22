use std::{error::Error, rc::Rc, collections::HashMap, cell::RefCell, sync::Mutex};

use aoc_lib::parsing::{parse_lines, Runnable};
use aoc_runner_api::SolverResult;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1},
    combinator::{complete, self},
    Parser,
    sequence::{preceded, tuple},
    branch::alt,
    error::VerboseError
};
use once_cell::sync::Lazy;

#[derive(Clone, Copy)]
enum UnaryOperator {
    Not
}

#[derive(Clone, Copy)]
enum BinaryOperator {
    And,
    Or,
    LeftShift,
    RightShift
}

enum Value<T> {
    Constant(u16),
    Variable(T)
}

type SubExpression = Rc<RefCell<NamedExpression>>;
type ExpressionTree<'a> = HashMap<&'a str, SubExpression>;

struct NamedExpression {
    name: String,
    expression: Expression
}

enum Expression {
    Unresolved,
    Literal(Value<SubExpression>),
    Unary(UnaryOperator, Value<SubExpression>),
    Binary(Value<SubExpression>, BinaryOperator, Value<SubExpression>)
}

enum UnresolvedExpression<'a> {
    Literal(Value<&'a str>),
    Unary(UnaryOperator, Value<&'a str>),
    Binary(Value<&'a str>, BinaryOperator, Value<&'a str>)
}

struct Assignment<'a> {
    target: &'a str,
    expression: UnresolvedExpression<'a>
}

impl Value<SubExpression> {
    fn evaluate(&self) -> Result<u16, Box<dyn Error>> {
        match self {
            Value::Constant(value) => Ok(*value),
            Value::Variable(variable) => variable.borrow().evaluate()
        }
    }
}

impl NamedExpression {
    fn evaluate(&self) -> Result<u16, Box<dyn Error>> {
        let mutex = (*EXPRESSION_CACHE).lock()?;
        let cache = mutex.get(&self.name);
        if cache.is_some() { return Ok(*cache.unwrap()); }
        
        drop(mutex);
        let result = self.expression.evaluate()?;

        let mut mutex = (*EXPRESSION_CACHE).lock()?;
        mutex.insert(self.name.clone(), result);
        
        return Ok(result);
    }
}

impl Expression {
    fn evaluate(&self) -> Result<u16, Box<dyn Error>> {
        match self {
            Expression::Unresolved => Err(String::from("Expression tree contained unresolved value during evaluation").into()),
            Expression::Literal(value) => value.evaluate(),
            Expression::Unary(operator, expression) => Ok(match operator {
                UnaryOperator::Not => !expression.evaluate()?
            }),
            Expression::Binary(left, operator, right) => Ok(match operator {
                BinaryOperator::And => left.evaluate()? & right.evaluate()?,
                BinaryOperator::Or => left.evaluate()? | right.evaluate()?,
                BinaryOperator::LeftShift => left.evaluate()? << right.evaluate()?,
                BinaryOperator::RightShift => left.evaluate()? >> right.evaluate()?
            })
        }
    }
}

static EXPRESSION_CACHE: Lazy<Mutex<HashMap<String, u16>>> = Lazy::new(|| Mutex::new(HashMap::new()));

fn parse_expression<'a>(input: &'a str) -> Result<Assignment<'a>, Box<dyn Error>> {
    use UnresolvedExpression::*;
    use BinaryOperator::*;
    use UnaryOperator::*;

    let constant = || complete::u16::<&str, VerboseError<&str>>.map(Value::Constant);
    let reference = || alpha1.map(Value::Variable);
    let value = || constant().or(reference());
    
    let binary_operator = alt((
        combinator::value(And, tag(" AND ")),
        combinator::value(Or, tag(" OR ")),
        combinator::value(LeftShift, tag(" LSHIFT ")),
        combinator::value(RightShift, tag(" RSHIFT "))
    ));

    let binary_operation = tuple((value(), binary_operator, value()))
        .map(|(left, operator, right)| Binary(left, operator, right));

    let unary_operator = combinator::value(Not, tag("NOT "));
    let unary_operation = tuple((unary_operator, value()))
        .map(|(operator, expression)| Unary(operator, expression));

    let literal = value().map(Literal);

    let expression = alt((
        unary_operation,
        binary_operation,
        literal
    ));

    let target = preceded(tag(" -> "), alpha1);
    let assignment = expression.and(target)
        .map(|(expression, target)| Assignment { expression, target });

    complete(assignment).run(input)
        .map_err(|err| err.into())
}

fn build_expression_tree<'a>(assignments: &'a Vec<Assignment>) -> Result<ExpressionTree<'a>, Box<dyn Error>> {
    use Value::*;

    let mut nodes = HashMap::<&str, SubExpression>::new();

    for assignment in assignments {
        nodes.insert(assignment.target, Rc::new(RefCell::new(
            NamedExpression { expression: Expression::Unresolved, name: assignment.target.to_owned() }
        )));
    }

    let find_node = |name: &str| nodes.get(name).ok_or(format!("'{}' is undefined", name));
    let map_value = |value: &Value<&str>| -> Result<Value<SubExpression>, Box<dyn Error>> {
        match value {
            Constant(x) => Ok(Constant(*x)),
            Variable(variable) =>  Ok(Variable(Rc::clone(find_node(variable)?)))
        }
    };

    let map_expression = |expression: &UnresolvedExpression| -> Result<Expression, Box<dyn Error>> {
        match expression {
            UnresolvedExpression::Literal(value) => Ok(Expression::Literal(map_value(value)?)),
            UnresolvedExpression::Unary(operator, value) => Ok(Expression::Unary(*operator, map_value(value)?)),
            UnresolvedExpression::Binary(left, operator, right) => Ok(Expression::Binary(map_value(left)?, *operator, map_value(right)?))
        }
    };

    for assignment in assignments {
        let node = nodes.get(assignment.target).unwrap();
        
        node.replace(NamedExpression { name: assignment.target.to_owned(), expression: map_expression(&assignment.expression)? });
    };

    return Ok(nodes);
}

fn evaulate_tree(tree: &ExpressionTree) -> Result<u16, Box<dyn Error>> {
    let result = tree.get("a")
        .ok_or(String::from("'a' was undefined"))?
        .borrow()
        .evaluate()?;

    Ok(result)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let assignments = parse_lines(parse_expression, input)?;
    let tree = build_expression_tree(&assignments)?;
    Ok(Box::new(evaulate_tree(&tree)?))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let assignments = parse_lines(parse_expression, input)?;
    let tree = build_expression_tree(&assignments)?;

    let b = tree.get("b")
        .ok_or(String::from("'b' was undefined"))?;

    let result = evaulate_tree(&tree)?;
    
    b.replace(NamedExpression {
        name: String::from("b"),
        expression: Expression::Literal(Value::Constant(result))
    });

    let mut cache = (*EXPRESSION_CACHE).lock()?;
    cache.clear();
    drop(cache);

    Ok(Box::from(evaulate_tree(&tree)?))
}