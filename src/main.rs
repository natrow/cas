// Main file

use cas::boolean_algebra::simplifying_tree::*;
use Operator::*;

fn main() {
    let axioms: RewriteSystem = vec![
        // a + 1 = 1
        RewriteRule { lhs: Box::new(Or(Box::new(Variable(String::from("a"))), Box::new(True))), rhs: Box::new(True) },
        // 1 + a = 1
        RewriteRule { lhs: Box::new(Or(Box::new(True), Box::new(Variable(String::from("a"))), )), rhs: Box::new(True) },
        // a + 0 = a
        RewriteRule { lhs: Box::new(Or(Box::new(Variable(String::from("a"))), Box::new(False))), rhs: Box::new(Variable(String::from("a"))) },
        // 0 + a = a
        RewriteRule { lhs: Box::new(Or(Box::new(False), Box::new(Variable(String::from("a"))))), rhs: Box::new(Variable(String::from("a"))) },
        // a + a = a
        RewriteRule { lhs: Box::new(Or(Box::new(Variable(String::from("a"))), Box::new(Variable(String::from("a"))))), rhs: Box::new(Variable(String::from("a")))}
    ];

    dbg!(&axioms);

    // TODO: finish Knuth-Bendix Completion to convert axioms into convergent TRS
    let trs = knuth_bendix_completion(&axioms);

    dbg!(&trs);

    let te = Box::new(True);
    let fe = Box::new(False);
    let ae = Box::new(Variable(String::from("a")));

    /*
        f = 1 + (0 + (0 + 0))
    */

    let exp = Or(te, Box::new(Or(fe.clone(), Box::new(Or(fe.clone(), fe.clone())))));

    dbg!(&exp);

    let exp = exp.simplify(&trs);

    dbg!(&exp);

    /*
        f = a + (a + 0)
    */

    let exp = Or(ae.clone(), Box::new(Or(ae.clone(), fe)));

    dbg!(&exp);

    let exp = exp.simplify(&trs);

    dbg!(&exp);
}
