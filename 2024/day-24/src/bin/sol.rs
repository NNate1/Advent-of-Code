use ::std::collections::HashMap;
use core::panic;

use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");

    // let input = include_str!("./example3.txt");

    let mut wires = parse(input);

    println!("part1: {}", part1(&mut wires));
    //
    println!("part2: {:?}", part2(&mut wires));
}

struct Wire<'a> {
    name: &'a str,
    gate: Gate<'a>,
    value: Option<bool>,
}

enum Gate<'a> {
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str),
    Value(bool),
}

fn parse(input: &str) -> HashMap<&str, Wire> {
    let mut gates = HashMap::new();
    // let mut gates: HashMap<String, Wire> = HashMap::new();

    let mut iter = input.lines();
    for (name, value) in iter
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once(' ').unwrap())
    {
        let name = name.strip_suffix(":").unwrap();
        let value = match value {
            "0" => false,
            "1" => true,
            _ => panic!(),
        };
        gates.insert(
            name,
            Wire {
                name,
                gate: Gate::Value(value),
                value: Some(value),
            },
        );
    }

    for (a, gate, b, _, name) in iter.flat_map(|line| line.split_whitespace()).tuples() {
        let (a, b) = if a < b { (a, b) } else { (b, a) };

        let gate = match gate {
            "AND" => Gate::And(a, b),
            "OR" => Gate::Or(a, b),
            "XOR" => Gate::Xor(a, b),
            _ => panic!(),
        };

        gates.insert(
            name,
            Wire {
                name,
                gate,
                value: None,
            },
        );
    }

    gates
}

fn part1(wires: &mut HashMap<&str, Wire>) -> usize {
    let targets: Vec<&str> = wires
        .keys()
        .filter(|name| name.starts_with("z"))
        .sorted()
        .rev()
        .copied()
        .collect();

    let mut result = 0;
    for target in targets {
        result <<= 1;
        if process_wire(wires, target) {
            result += 1;
        }
    }

    result
}

fn process_wire(wires: &mut HashMap<&str, Wire>, name: &str) -> bool {
    if let Some(value) = wires[name].value {
        return value;
    }

    let wire = &wires[name];
    let val = match wire.gate {
        Gate::And(a, b) => process_wire(wires, a) && process_wire(wires, b),

        Gate::Or(a, b) => process_wire(wires, a) || process_wire(wires, b),
        Gate::Xor(a, b) => process_wire(wires, a) ^ process_wire(wires, b),
        Gate::Value(v) => v,
    };

    wires.get_mut(name).unwrap().value = Some(val);
    val
}

fn part2(wires: &mut HashMap<&str, Wire>) {
    let results: Vec<&str> = wires
        .keys()
        .filter(|name| name.starts_with("z"))
        .sorted()
        .rev()
        .copied()
        .collect();

    let mut z: u64 = 0;
    for result in results {
        z <<= 1;
        if process_wire(wires, result) {
            z += 1;
        }
    }

    let mut x: u64 = 0;
    for bit in wires
        .iter()
        .sorted_by_key(|(name, _)| *name)
        .filter(|(name, _)| name.starts_with("x"))
        .map(|(_, wire)| wire.value.unwrap())
        .rev()
    {
        x <<= 1;
        if bit {
            x += 1;
        }
    }

    let mut y: u64 = 0;
    for bit in wires
        .iter()
        .sorted_by_key(|(name, _)| *name)
        .filter(|(name, _)| name.starts_with("y"))
        .map(|(_, wire)| wire.value.unwrap())
        .rev()
    {
        y <<= 1;
        if bit {
            y += 1;
        }
    }

    println!("x : {:#010b}\ny : {:#010b}", x, y,);
    println!("z : {:#010b}", z);

    // println!("x : {:#}\ny : {:#}", x, y,);
    // println!("z : {:#}", z);

    println!("t : {:#010b}", x + y);

    // write_to_mermaid_flowchart(wires);

    // visualize graph on https://dreampuf.github.io/GraphvizOnline/
    // and find gates that do not match a riple carry adder
    let _ = write_to_graphviz_dot(wires, "graph.dot");
}

// fn wrong(wires: &HashMap<&str, Wire>) {
//     let wrong = Vec::new();
//
//     for wire in wires.values() {
//         if wire.name.starts_with("z") {}
//     }
// }

fn write_to_mermaid_flowchart(wires: &mut HashMap<&str, Wire>) {
    // flowchart TD
    //     A[Christmas] -->|Get money| B(Go shopping)
    //     B --> C{Let me think}
    //     C -->|One| D[Laptop]
    //     C -->|Two| E[iPhone]
    //     C -->|Three| F[fa:fa-car Car]

    println!("flowchart TD");
    for (name, wire) in wires.iter().sorted_by_key(|(name, _)| *name) {
        let (gate, children) = match wire.gate {
            Gate::And(a, b) => ("AND", [a, b]),
            Gate::Or(a, b) => ("Or", [a, b]),
            Gate::Xor(a, b) => ("Xor", [a, b]),
            Gate::Value(_) => continue,
        };

        // println!("{0} --> {0}_gate{{{1}}}", name, gate);
        // for child in children {
        //     if !child.is_empty() {
        //         println!("{0}_gate --> {1}", name, child)
        //     }
        // }

        // if children.contains(&"x01") || children.contains(&"y02") {
        // if children.contains(&"jcr") || children.contains(&"rvp") {
        {
            println!("{0}_gate{{{1}}} --> {0}", name, gate);
            for child in children {
                println!("{1} --> {0}_gate", name, child);
            }
        }
    }
}

fn write_to_graphviz_dot(wires: &HashMap<&str, Wire>, output_path: &str) -> std::io::Result<()> {
    let mut file = File::create(output_path)?;

    writeln!(file, "digraph G {{")?;

    // Connect x input for better visualization
    writeln!(file, "  subgraph input_x {{")?;
    writeln!(file, "    node [style=filled,color=lightgrey];")?;
    write!(file, "x00")?;
    for i in 1..=wires.keys().filter(|name| name.starts_with("x")).count() {
        write!(file, " -> x{i:#02}")?;
    }
    writeln!(file, ";")?;
    writeln!(file, "  }}")?;

    // Connect y input for better visualization
    writeln!(file, "  subgraph input_y {{")?;
    writeln!(file, "    node [style=filled,color=lightgrey];")?;
    write!(file, "y00")?;
    for i in 1..=wires.keys().filter(|name| name.starts_with("y")).count() {
        write!(file, " -> y{i:#02}")?;
    }
    writeln!(file, ";")?;
    writeln!(file, "  }}")?;

    // And gates are green, (X AND Y), (X XOR Y ) AND Carry
    writeln!(file, "  subgraph gates_and {{")?;
    writeln!(file, "    node [style=filled,color=lightgreen];")?;
    for (name, wire) in wires.iter() {
        if let Gate::And(_, _) = wire.gate {
            writeln!(file, "    {};", name)?;
        }
    }
    writeln!(file, "  }}")?;

    // Or gates are yellow, (Carry OR Carry')
    writeln!(file, "  subgraph gates_or {{")?;
    writeln!(file, "    node [style=filled,color=yellow];")?;
    for (name, wire) in wires.iter() {
        if let Gate::Or(_, _) = wire.gate {
            writeln!(file, "    {};", name)?;
        }
    }
    writeln!(file, "  }}")?;

    // Xor gates are blue, (X XOR Y) XOR Carry
    writeln!(file, "  subgraph gates_xor {{")?;
    writeln!(file, "    node [style=filled,color=lightskyblue];")?;
    for (name, wire) in wires.iter() {
        if let Gate::Xor(_, _) = wire.gate {
            writeln!(file, "    {};", name)?;
        }
    }
    writeln!(file, "  }}")?;

    // Connect wires
    for (name, wire) in wires.iter() {
        let children = match wire.gate {
            Gate::And(a, b) | Gate::Or(a, b) | Gate::Xor(a, b) => [a, b],
            Gate::Value(_) => continue,
        };

        for child in children {
            if !child.is_empty() {
                writeln!(file, "  {} -> {};", child, name)?;
            }
        }
    }

    writeln!(file, "}}")?;

    Ok(())
}

fn riple_carry_adder(x: u64, y: u64) {
    // z = (x XOR y) XOR c,
    // c'= (x AND y) OR ((x XOR y) AND c)

    let x_xor_y = Wire {
        name: "x_xor_y",
        gate: Gate::Xor("x", "y"),
        value: None,
    };

    let z = Wire {
        name: "z",
        gate: Gate::Xor("c", "x_xor_y"),
        value: None,
    };

    // c'= (x AND y) OR ((x XOR y) AND c)

    let x_and_y = Wire {
        name: "x_and_y",
        gate: Gate::And("x", "y"),
        value: None,
    };

    let x_xor_y_and_c = Wire {
        name: "x_xor_y_and_c",
        gate: Gate::And("x_xor_y", "c"),
        value: None,
    };

    let carry = Wire {
        name: "carry",
        gate: Gate::Or("x_xor_y_and_c", "c"),
        value: None,
    };
}

use std::fs::File;
use std::io::Write;

fn super_write_to_mermaid_flowchart(
    wires: &HashMap<&str, Wire>,
    output_path: &str,
) -> std::io::Result<()> {
    let mut file = File::create(output_path)?;

    writeln!(file, "<!DOCTYPE html>")?;
    writeln!(file, "<html lang=\"en\">")?;
    writeln!(file, "<head>")?;
    writeln!(
        file,
        "<script src=\"https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js\"></script>"
    )?;
    writeln!(
        file,
        "<script>mermaid.initialize({{ startOnLoad: true }});</script>"
    )?;
    writeln!(file, "</head>")?;
    writeln!(file, "<body>")?;
    writeln!(file, "<div class=\"mermaid\">")?;
    writeln!(file, "flowchart TD")?;

    for (name, wire) in wires.iter().sorted_by_key(|(name, _)| *name) {
        let (gate, children) = match wire.gate {
            Gate::And(a, b) => ("AND", [a, b]),
            Gate::Or(a, b) => ("OR", [a, b]),
            Gate::Xor(a, b) => ("XOR", [a, b]),
            Gate::Value(_) => continue,
        };

        writeln!(file, "    {0}_gate{{{1}}} --> {0}", name, gate)?;
        for child in children {
            if !child.is_empty() {
                writeln!(file, "    {1} --> {0}_gate", name, child)?;
            }
        }
    }

    writeln!(file, "</div>")?;
    writeln!(file, "</body>")?;
    writeln!(file, "</html>")?;

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    // #[ignore]
    #[test]
    fn test_part1_ex1() {
        let mut input = parse(include_str!("./example1.txt"));
        let output = part1(&mut input);
        assert_eq!(output, 4);
    }

    #[test]
    fn test_part1_ex2() {
        let mut input = parse(include_str!("./example2.txt"));
        let output = part1(&mut input);
        assert_eq!(output, 2024);
    }

    // #[ignore]
    // #[test]
    // fn test_part2_ex1() {
    //     let input = parse(include_str!("./example1.txt"));
    //     let output = part2(&input);
    //     assert_eq!(output, "co,de,ka,ta");
    // }
}
