pub trait Gate<const I: usize, const O: usize> {
    fn get_i() -> usize {
        return I;
    }
    fn get_o() -> usize {
        return O;
    }
    fn process(&mut self, i: &mut Vec<bool>, o: &mut Vec<bool>);
}

pub enum GateTypes {
    AndGate(AndGate),
    OrGate(OrGate),
}
impl GateTypes {
    pub fn get_i(&self) -> usize {
        return match self {
            GateTypes::AndGate(_) => AndGate::get_i(),
            GateTypes::OrGate(_) => OrGate::get_i(),
        };
    }
    pub fn get_o(&self) -> usize {
        return match self {
            GateTypes::AndGate(_) => AndGate::get_o(),
            GateTypes::OrGate(_) => OrGate::get_o(),
        };
    }
    pub fn process(&mut self, i: &mut Vec<bool>, o: &mut Vec<bool>) {
        match self {
            GateTypes::AndGate(g) => g.process(i, o),
            GateTypes::OrGate(g) => g.process(i, o),
        }
    }
}

#[derive(Default)]
pub struct AndGate {
    pub state: bool,
}
impl Gate<2, 1> for AndGate {
    fn process(&mut self, i: &mut Vec<bool>, o: &mut Vec<bool>) {
        self.state = i.pop().unwrap() && i.pop().unwrap();
        o.push(self.state);
    }
}

#[derive(Default)]
pub struct OrGate {
    pub state: bool,
}
impl Gate<2, 1> for OrGate {
    fn process(&mut self, i: &mut Vec<bool>, o: &mut Vec<bool>) {
        self.state = i.pop().unwrap() || i.pop().unwrap();
        o.push(self.state);
    }
}
