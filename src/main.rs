// Main file

mod boolean_algebra;
use boolean_algebra::simplifying_tree::*;
use Operator::{Or,True,False};

fn main() {
    let rewrite_rules = create_rewrite_system();

    let te = Box::new(True);
    let fe = Box::new(False);

    /*
        f = 1 + (0 + (0 + 0))
    */

    let exp = Or(te, Box::new(Or(fe.clone(), Box::new(Or(fe.clone(), fe)))));

    dbg!(&exp);

    let exp = simplify(&exp, &rewrite_rules);

    dbg!(&exp);
}
