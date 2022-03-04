// Main file
use std::fmt;

trait ExpressionTrait : fmt::Debug + fmt::Display + Clone {
    fn simplify(&self) -> Expression;
    fn structurally_equivalent(&self, expr: &Expression) -> bool;
}

#[derive(Debug)]
enum Expression {
    // Generic numeric value, integers only
    Number(rug::Integer),
    // Sum of subexpressions
    Sum(Vec<Expression>),
    // Variables
    Variable(String),
    // Products
    Product(Vec<Expression>)
}

impl ExpressionTrait for Expression {
    fn simplify(&self) -> Expression {
        match self {
            // Numbers cannot be simplified
            Expression::Number(..) => {
                self.clone()
            }
            // Sums will attempt to add all numbers
            // also implements Identity Property (a + 0 = a)
            Expression::Sum(sub_exprs) => {
                let mut total = rug::Integer::new();
                let mut sum = Vec::new();
                
                for expr in sub_exprs {
                    match expr {
                        Expression::Number(value) => {
                            total += value;
                        }
                        other => {
                            sum.push(other.clone());
                        }
                    }
                }

                if total != rug::Integer::new() {
                    sum.push(Expression::Number(total));
                }

                if sum.len() == 1 {
                    sum[0].clone()
                } else if sum.len() == 0 {
                    Expression::Number(rug::Integer::new())
                } else {
                    Expression::Sum(sum)
                }
            }
            // Variables cannot be simplified
            Expression::Variable(..) => {
                self.clone()
            }
            // Products will multiply all numbers
            // and implements property (a * 1 = a)
            Expression::Product(sub_exprs) => {
                let mut total = rug::Integer::from(1);
                let mut prod = Vec::new();
                
                for expr in sub_exprs {
                    match expr {
                        Expression::Number(value) => {
                            total *= value;
                        }
                        other => {
                            prod.push(other.clone());
                        }
                    }
                }

                if total != rug::Integer::from(1) {
                    prod.push(Expression::Number(total));
                }

                if prod.len() == 1 {
                    prod[0].clone()
                } else if prod.len() == 0 {
                    Expression::Number(rug::Integer::from(1))
                } else {
                    Expression::Sum(prod)
                }
            }
        }
    }

    fn structurally_equivalent(&self, expr: &Expression) -> bool {
        match (self, expr) {
            (Expression::Number(val1), Expression::Number(val2)) => {
                val1 == val2
            }
            (Expression::Sum(sub_exprs1), Expression::Sum(sub_exprs2)) => {
                unordered_eq(sub_exprs1, sub_exprs2)
            }
            (Expression::Variable(name1), Expression::Variable(name2)) => {
                name1 == name2
            }
            (Expression::Product(sub_exprs1), Expression::Product(sub_exprs2)) => {
                unordered_eq(sub_exprs1, sub_exprs2)
            }
            _ => {
                false
            }
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Numbers simply display their values
            Expression::Number(value) => {
                write!(f, "{}", value)
            }
            // Sums print subexpressions separated by " + "
            Expression::Sum(sub_exprs) => {
                let mut string = String::new();

                for (i, expr) in sub_exprs.iter().enumerate() {
                    string += &expr.to_string();
                    if i < sub_exprs.len() - 1 {
                        string += " + ";
                    }
                }

                write!(f, "{}", string)
            }
            // Variables print their name
            Expression::Variable(name) => {
                write!(f, "{}", name)
            }
            // Products print subexpressions separated by " * "
            Expression::Product(sub_exprs) => {
                let mut string = String::new();

                for (i, expr) in sub_exprs.iter().enumerate() {
                    string += &expr.to_string();
                    if i < sub_exprs.len() - 1 {
                        string += " * ";
                    }
                }

                write!(f, "{}", string)
            }
        }
    }
}

impl Clone for Expression {
    fn clone(&self) -> Expression {
        match self {
            // Numbers need to clone their value
            Expression::Number(value) => {
                Expression::Number(value.clone())
            }
            // Sums need to clone all of their subexpressions
            Expression::Sum(sub_exprs) => {
                Expression::Sum(sub_exprs.clone())
            }
            // Variables need to clone their name
            Expression::Variable(name) => {
                Expression::Variable(name.clone())
            }
            // Products need to clone all of their subexpressions
            Expression::Product(sub_exprs) => {
                Expression::Product(sub_exprs.clone())
            }
        }
    }
}

fn unordered_eq(exprs1: &Vec<Expression>, exprs2: &Vec<Expression>) -> bool {
    if exprs1.len() != exprs2.len() {
        return false
    }

    let mut used_index = Vec::with_capacity(exprs1.len());

    'outer: for (i, expr1) in exprs1.iter().enumerate() {
        for expr2 in exprs2 {
            if used_index[i]  {continue;}
            if expr1.structurally_equivalent(expr2) {
                used_index[i] = true;
                continue 'outer;
            }
        }
        // could not find unused expr2 in exprs1
        return false
    }

    true
}

fn main() {
    let x = Expression::Number(rug::Integer::from(9));
    let y = Expression::Number(rug::Integer::from(10));
    let z = Expression::Number(rug::Integer::from(2));

    let f = Expression::Sum(vec![x, y, z]);

    dbg!(&f);

    println!("f is {}", &f);

    let f = f.simplify();

    dbg!(&f);

    println!("f is {}", &f);
}
