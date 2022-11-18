use anyhow::*;

use std::convert::TryInto;
use std::ops;

pub type MemItem = i64;
pub type Address = usize;
pub type IntCode = Vec<MemItem>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IntCodeComputer {
    memory: IntCode,
    ip: Address,
    halted: bool,
}

impl IntCodeComputer {
    pub fn new(code: impl Into<IntCode>) -> Self {
        Self::new_with_extra_capacity(code, 16)
    }

    pub fn new_with_extra_capacity(code: impl Into<IntCode>, space: usize) -> Self {
        let mut memory = code.into();
        memory.resize_with(memory.len() + space, Default::default);
        Self {
            memory,
            ip: 0,
            halted: false,
        }
    }

    pub fn read(&self, addr: Address) -> Option<MemItem> {
        self.memory.get(addr).copied()
    }

    fn read_res(&self, addr: Address) -> Result<MemItem> {
        self.memory
            .get(addr)
            .copied()
            .ok_or_else(|| format_err!("Failed to read from address {}.", addr))
    }

    fn read_addr(&self, addr: Address) -> Result<Address> {
        let value = self.read_res(addr)?;

        Ok(value.try_into()?)
    }

    pub fn write(&mut self, addr: Address, value: MemItem) -> Result<()> {
        let len = self.memory.len();
        let mem = self.memory.get_mut(addr).ok_or_else(|| format_err!(
            "Cannot write to address {}, memory has size {}",
            addr,
            len,
        ))?;
        *mem = value;
        Ok(())
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    fn arith_op<F>(&mut self, op: F, arg1: Address, arg2: Address, dest: Address) -> Result<()>
    where
        F: FnOnce(MemItem, MemItem) -> MemItem,
    {
        let val1 = self.read_res(arg1)?;
        let val2 = self.read_res(arg2)?;

        self.write(dest, op(val1, val2))?;

        self.ip += 4;

        Ok(())
    }

    // steps the state and returns whether the program halted
    // or whether an error occured
    pub fn step(&mut self) -> Result<bool> {
        ensure!(!self.is_halted(), "Cannot step a halted computer.");

        let instr = self.read_res(self.ip)?;

        macro_rules! arg {
            ($num:expr) => {
                self.read_addr(self.ip + $num)?
            };
        }

        match instr {
            1 => { self.arith_op(ops::Add::add, arg!(1), arg!(2), arg!(3))?; }
            2 => { self.arith_op(ops::Mul::mul, arg!(1), arg!(2), arg!(3))?; }
            99 => {
                self.halted = true;
            }
            _ => {
                bail!(
                    "Encountered unknown instruction {} at address {}.",
                    instr,
                    self.ip
                );
            }
        };

        Ok(self.halted)
    }

    // run the code until halted
    pub fn run(&mut self) -> Result<()> {
        loop {
            if self.step()? {
                break;
            }
        }

        Ok(())
    }
}

impl std::str::FromStr for IntCodeComputer {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .trim()
            .split(',')
            .map(|num| num.parse::<MemItem>())
            .collect::<Result<IntCode, Self::Err>>()?;

        Ok(IntCodeComputer::new(instructions))
    }
}
