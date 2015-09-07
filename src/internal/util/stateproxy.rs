use {MatchCapture, MatchState};
use std;

pub struct MatchStateProxy<'a> {
    state: &'a mut MatchState,
    captured: Vec<MatchCapture>,
}

impl<'a> MatchStateProxy<'a> {
    pub fn new(state: &'a mut MatchState) -> MatchStateProxy<'a> {
        MatchStateProxy { state: state, captured: Vec::new(), }
    }

    pub fn our_captures(&self) -> usize {
        self.captured.len()
    }

    pub fn inject(&mut self) {
        for v in self.captured.iter() {
            self.state.push_capture(*v)
        }
    }
}

impl<'a> MatchState for MatchStateProxy<'a> {
    fn pos(&self) -> usize {
        self.state.pos()
    }

    fn set_pos(&mut self, pos: usize) -> bool {
        self.state.set_pos(pos)
    }

    fn max_pos(&self) -> usize {
        self.state.max_pos()
    }

    fn has_next(&self) -> bool {
        self.state.has_next()
    }

    unsafe fn unsafe_next(&mut self) -> u8 {
        self.state.unsafe_next()
    }

    fn next(&mut self) -> Option<u8> {
        self.state.next()
    }

    fn get(&self) -> u8 {
        self.state.get()
    }

    fn captures(&self) -> usize {
        self.state.captures() + self.captured.len()
    }

    fn get_capture(&self, index: usize) -> Option<MatchCapture> {
        if index >= self.state.captures() {
            match self.captured.get(index - self.state.captures()) {
                Some(c) => Some(*c),
                None => None,
            }
        } else {
            return self.state.get_capture(index)
        }
    }

    fn push_capture(&mut self, captured: MatchCapture) {
        assert!(self.our_captures() < std::usize::MAX - self.state.captures(), "too many captures");
        self.captured.push(captured)
    }
}

pub fn new<'a>(state: &'a mut MatchState) -> MatchStateProxy<'a> {
    MatchStateProxy::new(state)
}
