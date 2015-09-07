// Quantifiers
// ... ARE A PAIN IN THE ASS TO DO ;_;

// TODO

use std::fmt;
use {PatternElement, MatchState, CompareResult, Next};
use internal;

#[derive(Copy, Clone, Debug)]
pub enum Quantification {
    Greedy,
    NonGreedy,
    Optional
}

pub struct Quantifier {
    quantification: Quantification,
    quantified: Box<PatternElement>,
}

impl Quantifier {
    pub fn new(quantification: Quantification, quantified: Box<PatternElement>) -> Quantifier {
        Quantifier { quantification: quantification, quantified: quantified }
    }
}

impl PatternElement for Quantifier {
    fn compare_next(&self, state: &mut MatchState, next: Option<&Next>) -> CompareResult {
        match self.quantification {
            Quantification::Greedy => { // TODO check
                // TODO rewrite; this is wrong
                match self.quantified.compare(state) {
                    CompareResult::Match(0) => {},
                    r => { return r }
                }
                let mut matched: Vec<usize> = Vec::new();
                matched.push(state.pos());
                loop {
                    match self.quantified.compare(state) {
                        CompareResult::Match(0) => { matched.push(state.pos()) },
                        _ => { break },
                    }
                }
                let mut last = CompareResult::Match(0);
                for backtrack in matched.iter().rev() {
                    state.set_pos(*backtrack);
                    if let Some(n) = next {
                        match n.compare(state) {
                            CompareResult::Match(0) => { return CompareResult::Match(0) },
                            r => { last = r },
                        }
                    }
                }
                last
            },
            Quantification::NonGreedy => {
                // TODO test
                if self.quantified.handle_next() {
                    loop {
                        match self.quantified.compare(state) {
                            CompareResult::Match(0) if next.is_some() => {
                                let pos = state.pos();
                                match next.unwrap().compare(state) {
                                    CompareResult::Match(0) => { return CompareResult::Match(0) },
                                    _ => {},
                                }
                                state.set_pos(pos);
                            },
                            r => { return r },
                        }
                    }
                } else {
                    loop {
                        match self.quantified.compare(state) {
                            CompareResult::Match(0) if next.is_some() => {
                                let pos = state.pos();
                                match next.unwrap().compare(state) {
                                    CompareResult::Match(0) => { return CompareResult::Match(0) },
                                    _ => {},
                                }
                                state.set_pos(pos);
                            },
                            r => { return r },
                        }
                    }
                }
            },
            Quantification::Optional => {
                // TODO test?
                let start = state.pos();
                let res;
                {
                    let mut proxy = internal::util::stateproxy::new(state);
                    res = self.quantified.compare_next(&mut proxy, next);
                    if let CompareResult::Match(0) = res {
                        proxy.inject();
                        return res;
                    }
                }
                state.set_pos(start);
                if let Some(n) = next { return n.compare(state) }
                CompareResult::Match(0)
            },
        }
    }

    fn handle_next(&self) -> bool {
        true
    }
}

impl fmt::Display for Quantifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.quantified));
        match self.quantification {
            Quantification::Greedy => write!(f, "+"),
            Quantification::NonGreedy => write!(f, "-"),
            Quantification::Optional => write!(f, "?"),
        }
    }
}

// NextHack: DON'T DO THIS!

struct NextHack<'a> {
    quantification: Quantification,
    quantified: &'a PatternElement,
}

impl<'a> NextHack<'a> {
    fn new(q: &'a Quantifier) -> NextHack<'a> {
        NextHack { quantification: q.quantification, quantified: &*q.quantified }
    }
}

impl<'a> PatternElement for NextHack<'a> {
    fn compare_next(&self, state: &mut MatchState, next: Option<&Next>) -> CompareResult {
        CompareResult::Match(0)
    }

    fn handle_next(&self) -> bool {
        true
    }
}

impl<'a> fmt::Display for NextHack<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "[Quantifier hack cannot be printed, as it is not part of a pattern.]")
    }
}
