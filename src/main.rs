
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

pub struct registers{
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8
}

pub struct FlagsRegister{
    zero: bool,
    subtract:bool,
    half_carry:bool,
    carry:bool
}

impl Registers {
    fn get_af(&self)->u16{
        (self.a as u16) << 8 | self.f as u16
    }

    fn set_af(&mut self, value: u16){
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = (value & 0xFF) as u8;
    }
    fn get_bc(&self)->u16{
        (self.b as u16) << 8 | self.c as u16
    }

    fn set_bc(&mut self, value: u16){
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }
    fn get_de(&self)->u16{
        (self.d as u16) << 8 | self.e as u16
    }

    fn set_de(&mut self, value: u16){
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn get_hl(&self)->u16{
        (self.h as u16) << 8 | self.l as u16
    }

    fn set_hl(&mut self, value: u16){
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

impl From<FlagsRegister> for u8{
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero {1} else {0})  << ZERO_FLAG_BYTE_POSITION |
            (if flag.subtract {1} else {0}) << SUBTRACT_FLAG_BYTE_POSITION |
            (if flag.half_carry {1} else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION|
            (if flag.carry {1} else {0}) << CARRY_FLAG_BYTE_POSITION
    }
}

impl From<u8> for FlagsRegister{
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) &0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) &0b1) !=0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) &0b1) != 0;

        FlagsRegister{
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}

enum Instruction {
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    ADDHL(ArithmeticTarget),
    SUB(ArithmeticTarget),
}
enum ArithmeticTarget{
    A, B, C, D, E, H, L,
}

impl CPU{
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                }
            }
            Instruction::ADDHL(target) =>{
                let value =
                self.addhl(value)
            }
        }
    }

    fn add(&mut self, value:u8) ->u8{
        let(new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value ==0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

    fn addhl(&mut self, value: u8 ){
        let(new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.h.zero = new_value ==0;
        self.registers.h.subtract = false;
        self.registers.h.carry = did_overflow;
        self.registers.h.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.set_hl(self.registers.get_hl(self) + new_value[value])
    }

    fn adc(&mut self, value:u8) ->u8{
        let(new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value ==0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value + did_overflow as u8
    }
    fn sub(&mut self, value: u8) -> u8{
        let(new_value, did_borrow) = self.registers.a.borrowing_sub(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_borrow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
}

fn main() {

}
