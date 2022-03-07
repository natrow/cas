// This enum encapsulates the types of operations allowed
// within boolean algebra
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Operator {
    Variable(String),
    True,
    False,
    Or(Box<Operator>, Box<Operator>),
    And(Box<Operator>, Box<Operator>),
    Not(Box<Operator>)
}

#[derive(Debug, Clone)]
pub struct RewriteRule {
    pub lhs: Box<Operator>,
    pub rhs: Box<Operator>
}

pub type RewriteSystem = Vec<RewriteRule>;

pub fn knuth_bendix_completion(axioms: &RewriteSystem) -> RewriteSystem {
    let mut rule_set = Vec::new();

    // while axiom set is not empty
    // select & remove axiom from the axiom set
    for axiom in axioms {
        // normalize the selected axiom
        let axiom = axiom.normalize(&rule_set);

        // if the normalized axiom is not of the form x=x then
        if !axiom.is_trivial() {
            // order the rule
            // this is implied, lhs > rhs

            // introduce rule to rule set
            rule_set.push(axiom);

            // superimpose new rule to all existing rules (including itself)

            // TODO

            // introduce each critical pair into axiom set

            // TODO
        }
    }

    rule_set
}

impl Operator {
    // Simplify attempts to match a rule from the rewrite system and apply it
    pub fn simplify(&self, rewrite_system: &RewriteSystem) -> Box<Self> {
        // simplify children
        let exp = match self {
            Operator::Or(l, r) => Operator::Or(l.simplify(rewrite_system), r.simplify(rewrite_system)),
            Operator::And(l, r) => Operator::And(l.simplify(rewrite_system), r.simplify(rewrite_system)),
            Operator::Not(s) => Operator::Not(s.simplify(rewrite_system)),
            _ => {
                self.clone()
            }
        };

        // attempt to simplify self
        for rule in rewrite_system {
            let mut substitutions = HashMap::new();

            if exp.matches_rule(rule.lhs.as_ref(), &mut substitutions) {
                return exp.apply_rule(rule.rhs.as_ref(), &substitutions)
            }
        }

        // couldn't simplify self, return simplified children
        Box::new(exp)
    }

    // Exact matches are when two expressions are identical
    fn matches_exact(&self, expression: &Operator) -> bool {
        match (self, expression) {
            // if types are the same, match verbatim
            (Operator::Variable(sn), Operator::Variable(en)) => sn == en,
            (Operator::True, Operator::True) => true,
            (Operator::False, Operator::False) => true,
            (Operator::Or(sl, sr), Operator::Or(el, er)) => sl.matches_exact(el) && sr.matches_exact(er),
            (Operator::And(sl, sr), Operator::And(el, er)) => sl.matches_exact(el) && sr.matches_exact(er),
            (Operator::Not(sc), Operator::Not(ec)) => sc.matches_exact(ec),
            // otherwise they don't match
            (_, _) => false
        }
    }

    // Rule matches are when the expression follows the "pattern" laid out by the rule
    // it also populates the substitutions that the rule creates
    fn matches_rule(&self, rule: &Operator, substitutions: &mut HashMap<String, Box<Operator>>) -> bool {
        match (rule, self) {
            // Identities are easy, they just match themselves exactly
            (Operator::True, Operator::True) => true,
            (Operator::False, Operator::False) => true,
            // Operators match recursively
            (Operator::Or(rl, rr), Operator::Or(el, er)) => el.matches_rule(rl, substitutions) && er.matches_rule(rr, substitutions),
            (Operator::And(rl, rr), Operator::And(el, er)) => el.matches_rule(rl, substitutions) && er.matches_rule(rr, substitutions),
            (Operator::Not(rc), Operator::Not(ec)) => ec.matches_rule(rc, substitutions),
            // Variables are tricky, the names represent unique substitutions
            (Operator::Variable(rn), _) => if substitutions.contains_key(rn) {
                self.matches_exact(substitutions[rn].as_ref())
            } else {
                substitutions.insert(rn.clone(), Box::new(self.clone()));
                true
            },
            // Anything else doesn't match
            (_, _) => false
        }
    }

    fn apply_rule(&self, rule: &Operator, substitutions: &HashMap<String, Box<Operator>>) -> Box<Self> {
        Box::new(match rule {
            // identities do a trivial copy
            Operator::True => Operator::True,
            Operator::False => Operator::False,
            // operations apply the rule recursively
            Operator::Or(rl, rr) => if let Operator::Or(el, er) = self {
                Operator::Or(el.apply_rule(rl.as_ref(), &substitutions), er.apply_rule(rr.as_ref(), &substitutions))
            } else { 
                panic!("illegally applied unmatched rule!")
            }
            Operator::And(rl, rr) => if let Operator::And(el, er) = self {
                Operator::And(el.apply_rule(rl.as_ref(), &substitutions), er.apply_rule(rr.as_ref(), &substitutions))
            } else { 
                panic!("illegally applied unmatched rule!")
            }
            Operator::Not(rc) => if let Operator::Not(ec) = self {
                Operator::Not(ec.apply_rule(rc.as_ref(), &substitutions))
            } else { 
                panic!("illegally applied unmatched rule!")
            }
            // variables make substitutions using the HashMap
            Operator::Variable(name) => {
                (*substitutions[name]).clone()
            }
        })
    }
}

impl RewriteRule {
    fn normalize(&self, rewrite_system: &RewriteSystem) -> Self {
        RewriteRule { lhs: self.lhs.simplify(&rewrite_system), rhs: self.rhs.simplify(&rewrite_system) }
    }

    fn is_trivial(&self) -> bool {
        self.lhs.matches_exact(self.rhs.as_ref())
    }
}
