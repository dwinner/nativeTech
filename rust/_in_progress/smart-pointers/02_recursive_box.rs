// 02_recursive_box.rs
use std::fmt;

// A recursive enum representing a simple arithmetic expression.
// `Box<T>` is CRITICAL here. Without it, Rust wouldn't know how much memory 
// to allocate for `Expression` because an `Add` contains more `Expression`s, 
// creating an infinite size. `Box` provides a fixed pointer size.
#[derive(Debug)]
enum Expression {
    Value(i32),
    Add(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Negate(Box<Expression>),
}

// A custom error type for our evaluation function.
#[derive(Debug)]
pub enum EvaluationError {
    Overflow,
}

// Implement Display for user-friendly error messages.
impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvaluationError::Overflow => write!(f, "Arithmetic overflow occurred during evaluation"),
        }
    }
}

// Helper functions for building the expression tree ergonomically.
fn val(v: i32) -> Box<Expression> {
    Box::new(Expression::Value(v))
}

fn add(left: Box<Expression>, right: Box<Expression>) -> Box<Expression> {
    Box::new(Expression::Add(left, right))
}

fn multiply(left: Box<Expression>, right: Box<Expression>) -> Box<Expression> {
    Box::new(Expression::Multiply(left, right))
}

fn negate(expr: Box<Expression>) -> Box<Expression> {
    Box::new(Expression::Negate(expr))
}

// A robust, recursive function to evaluate the expression tree.
// It returns a Result to handle potential overflows.
fn evaluate(expr: &Expression) -> Result<i32, EvaluationError> {
    match expr {
        Expression::Value(v) => Ok(*v),
        Expression::Add(left, right) => {
            // Recursively evaluate sub-expressions, propagating errors with `?`.
            let left_val = evaluate(left)?;
            let right_val = evaluate(right)?;
            // Use checked_add, which returns an Option, then convert to Result.
            left_val.checked_add(right_val).ok_or(EvaluationError::Overflow)
        }
        Expression::Multiply(left, right) => {
            let left_val = evaluate(left)?;
            let right_val = evaluate(right)?;
            // Use checked_mul for safe multiplication.
            left_val.checked_mul(right_val).ok_or(EvaluationError::Overflow)
        }
        Expression::Negate(inner_expr) => {
            let val = evaluate(inner_expr)?;
            // Use checked_neg for safe negation.
            val.checked_neg().ok_or(EvaluationError::Overflow)
        }
    }
}

fn main() {
    // Build an expression for: (5 + 10) * -2
    let expr = multiply(
        add(val(5), val(10)),
        negate(val(2)),
    );

    println!("--- Expression 1: (5 + 10) * -2 ---");
    match evaluate(&expr) {
        Ok(result) => println!("Evaluated: {}", result), // Expected: -30
        Err(e) => println!("Error: {}", e),
    }

    // Build an expression for: -(3 + (4 * 5))
    let expr2 = negate(
        add(
            val(3),
            multiply(val(4), val(5)),
        )
    );

    println!("\n--- Expression 2: -(3 + (4 * 5)) ---");
    match evaluate(&expr2) {
        Ok(result) => println!("Evaluated: {}", result), // Expected: -23
        Err(e) => println!("Error: {}", e),
    }

    // Build an expression designed to overflow an i32
    let overflow_expr = add(val(i32::MAX), val(1));

    println!("\n--- Expression 3: i32::MAX + 1 (Overflow Test) ---");
    match evaluate(&overflow_expr) {
        Ok(result) => println!("Evaluated: {}", result),
        Err(e) => println!("Correctly caught error: {}", e), // Expected: Arithmetic overflow...
    }
}