use std::collections::HashMap;
use std::fmt;

pub struct InterpreterState {
    code: String,
    code_ptr: usize,
    mem: HashMap<i32, i8>,
    mem_ptr: i32,
}

pub enum InstructionResult {
    StopExecution,
    Continue,
    OutputChar(i8),
    BracketError(usize),
}

impl InterpreterState {
    pub fn new(code: String) -> InterpreterState {
        InterpreterState {
            code: code,
            code_ptr: 0,
            mem: HashMap::new(),
            mem_ptr: 0,
        }
    }

    pub fn get_code(&self) -> &String {
        &self.code
    }

    pub fn handle_instruction<InFn: Fn() -> i8>(
        &mut self,
        input_handler: InFn,
    ) -> InstructionResult {
        if self.code_ptr >= self.code.len() {
            return InstructionResult::StopExecution;
        }
        match &self.code[self.code_ptr..(self.code_ptr + 1)] {
            "+" => {
                self.mem
                    .entry(self.mem_ptr)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
            "-" => {
                self.mem
                    .entry(self.mem_ptr)
                    .and_modify(|e| *e -= 1)
                    .or_insert(-1);
            }
            ">" => self.mem_ptr += 1,
            "<" => self.mem_ptr -= 1,
            "," => {
                self.mem.insert(self.mem_ptr, input_handler());
            }
            "." => {
                self.code_ptr += 1;
                return InstructionResult::OutputChar(
                    self.mem.get(&self.mem_ptr).unwrap_or(&0).clone(),
                );
            }
            "[" => {
                let mut bracket_counter = 1;
                let mut index = self.code_ptr;
                while bracket_counter > 0 && index < self.code.len() {
                    index += 1;
                    match &self.code[index..(index + 1)] {
                        "[" => bracket_counter += 1,
                        "]" => bracket_counter -= 1,
                        _ => {}
                    }
                }
                if index >= self.code.len() {
                    return InstructionResult::BracketError(self.code_ptr);
                }
                self.code_ptr = index - 1;
            }
            "]" => {
                let mut bracket_counter = 1;
                let mut index = self.code_ptr;
                while bracket_counter > 0 && index >= 0 {
                    index -= 1;
                    match &self.code[index..(index + 1)] {
                        "]" => bracket_counter += 1,
                        "[" => bracket_counter -= 1,
                        _ => {}
                    }
                }
                if index < 0 {
                    return InstructionResult::BracketError(self.code_ptr);
                }
                self.code_ptr = index;
            }
            _ => {}
        };
        self.code_ptr += 1;
        return InstructionResult::Continue;
    }
}

impl fmt::Display for InterpreterState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Code: {}\n", self.code))?;
        for _ in 0..(self.code_ptr + 6) {
            write!(f, " ")?;
        }
        write!(f, "^\n")?;
        f.write_fmt(format_args!("Memory pointer at: {}\n", self.mem_ptr))?;
        f.debug_map().entries(self.mem.iter()).finish()
    }
}

pub struct InterpreterRun<InFn> {
    state: InterpreterState,
    error: Option<usize>,
    input_fn: InFn,
}

impl<InFn> InterpreterRun<InFn>
where
    InFn: Fn() -> i8,
{
    pub fn from_state(state: InterpreterState, input_handler: InFn) -> Self {
        InterpreterRun {
            state: state,
            input_fn: input_handler,
            error: Option::None,
        }
    }

    pub fn new(code: String, input_handler: InFn) -> Self {
        Self::from_state(InterpreterState::new(code), input_handler)
    }
}

impl<InFn> Iterator for InterpreterRun<InFn>
where
    InFn: Fn() -> i8,
{
    type Item = Option<i8>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state.handle_instruction(&self.input_fn) {
            InstructionResult::StopExecution => Option::None,
            InstructionResult::Continue => Option::Some(Option::None),
            InstructionResult::OutputChar(c) => Option::Some(Option::Some(c)),
            InstructionResult::BracketError(err) => {
                self.error = Option::Some(err);
                Option::None
            }
        }
    }
}
