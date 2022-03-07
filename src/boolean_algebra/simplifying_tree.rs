
// This enum encapsulates the types of operations allowed
// within boolean algebra
#[derive(Debug, Clone)]
pub enum Operator {
    Variable(String),
    True,
    False,
    Or(Box<Operator>, Box<Operator>),
    And(Box<Operator>, Box<Operator>),
    Not(Box<Operator>)
}

type RewriteRule = fn(&Operator) -> Option<Operator>;
type RewriteSystem = Vec<RewriteRule>;

pub fn simplify(tree: &Operator, rewrite_system: &RewriteSystem) -> Operator {
    match tree {
        // For each operator, simplify the children, apply rewrites,
        // then attempt to simplify again until done

        Operator::Or(left, right) => {
            let new_left = simplify(left, rewrite_system);
            let new_right = simplify(right, rewrite_system);
            let new_tree = Operator::Or(Box::new(new_left), Box::new(new_right));

            for rule in rewrite_system {
                match rule(&new_tree) {
                    Some(rewritten_tree) => return simplify(&rewritten_tree, rewrite_system),
                    None => {}
                }
            }

            new_tree
        }

        Operator::And(left, right) => {
            let new_left = simplify(left, rewrite_system);
            let new_right = simplify(right, rewrite_system);
            let new_tree = Operator::And(Box::new(new_left), Box::new(new_right));

            for rule in rewrite_system {
                match rule(&new_tree) {
                    Some(rewritten_tree) => return simplify(&rewritten_tree, rewrite_system),
                    None => {}
                }
            }

            new_tree
        }

        Operator::Not(var) => {
            let new_var = simplify(var, rewrite_system);
            let new_tree = Operator::Not(Box::new(new_var));

            for rule in rewrite_system {
                match rule(&new_tree) {
                    Some(rewritten_tree) => return simplify(&rewritten_tree, rewrite_system),
                    None => {}
                }
            }

            new_tree
        }

        // Variables, true and false cannot be simplified...
        _ => tree.clone()
    }
}

fn or_identities(tree: &Operator) -> Option<Operator> {
    match tree {
        Operator::Or(left, right) => {
            match **left {
                Operator::True => return Option::Some(Operator::True),
                Operator::False => return Option::Some(*right.clone()),
                _ => {}
            }
            match **right {
                Operator::True => return Option::Some(Operator::True),
                Operator::False => return Option::Some(*left.clone()),
                _ => {}
            }
        },
        _ => {}
    }
    None
}

pub fn create_rewrite_system() -> RewriteSystem {
    vec![or_identities]
}
