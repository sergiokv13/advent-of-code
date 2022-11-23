use std::collections::HashMap;
use std::{ io };
use crate::utils::{self};

fn get_wire_value(wire_key: String, wire_instr: &HashMap<String,String>, memo: &mut HashMap<String,u16>) -> u16 {
    if memo.contains_key(&wire_key) {
        return *memo.get(&wire_key).unwrap();
    }

    let instruction = wire_instr.get(&wire_key).unwrap();

    fn get_vals (sep: &str, instruction: &str, wire_instr: &HashMap<String,String>, memo: &mut HashMap<String,u16>) -> (u16, u16) {
        let sp : Vec<String> = instruction.split(sep).map(String::from).collect();
        let num_val_1 = if sp[0].parse::<u16>().is_ok() { sp[0].parse::<u16>().unwrap() } else { get_wire_value(format!("{}", sp[0]), wire_instr, memo) };
        let num_val_2 = if sp[1].parse::<u16>().is_ok() { sp[1].parse::<u16>().unwrap() } else { get_wire_value(format!("{}", sp[1]), wire_instr, memo) };            
        return (num_val_1, num_val_2);
    }

    let res: u16 = (|| {
        match instruction {
            x if x.contains("AND") => {
                let (n1,n2) = get_vals(" AND ", &instruction, wire_instr, memo);
                return n1 & n2;
            },
            x if x.contains("OR") => {
                let (n1,n2) = get_vals(" OR ", &instruction, wire_instr, memo);
                return n1 | n2;
            },
            x if x.contains("LSHIFT") => {
                let (n1,n2) = get_vals(" LSHIFT ", &instruction, wire_instr, memo);
                return n1 << n2;
            },
            x if x.contains("RSHIFT") => {
                let (n1,n2) = get_vals(" RSHIFT ", &instruction, wire_instr, memo);
                return n1 >> n2;
            },
            x if x.contains("NOT") => {
                let val = instruction.replace("NOT ", "");
                let num_val = if val.parse::<u16>().is_ok() { val.parse::<u16>().unwrap() } else { get_wire_value(val, wire_instr, memo) };
                return !num_val
            },
            _ => { return if instruction.parse::<u16>().is_ok() { instruction.parse::<u16>().unwrap() } else { get_wire_value(instruction.to_string(), wire_instr, memo) }; },
        }
    })();
    memo.insert(wire_key, res);
    return res;

}

pub fn solve() -> Result<(), io::Error>{
    let lines = utils::get_lines("day_7");
    let mut wire_instr: HashMap<String,String> = HashMap::new(); // Store the instruction to get to wire

    let mut memo: HashMap<String,u16> = HashMap::new(); // Store the calculated value

    for line in lines {
        let splitted : Vec<String> = line.split(" -> ").map(String::from).collect();
        wire_instr.insert(format!("{}",splitted[1]), format!("{}", splitted[0]));
    }

    println!("First Star: {}", get_wire_value(String::from("a"), &wire_instr, &mut memo));

    return Ok(())
}