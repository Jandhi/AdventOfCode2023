use std::{collections::{HashSet, HashMap}, hash::BuildHasher};

fn main() {
    let input = include_str!("input.txt");

    let module_infos : Vec<(&str, ModuleType, Vec<&str>)> = input.split("\n")
        .map(str::trim)
        .map(|line| {
            let parts : Vec<&str> = line.split(" -> ").collect();
            let outputs = parts[1].split(", ").collect();

            if parts[0].starts_with("%") {
                return (
                    &parts[0][1..], 
                    ModuleType::FlipFlop(FlipFlop { is_on: false }),
                    outputs,
                )
            }

            if parts[0].starts_with("&") {
                return (
                    &parts[0][1..],
                    ModuleType::Conjuction(Conjuction { last_inputs: HashMap::new() }),
                    outputs
                )
            }

            return (
                &parts[0],
                ModuleType::Broadcast,
                outputs
            )
        })
        .collect();

    let mut names : Vec<&str> = module_infos.iter()
        .map(|(name, _, _)| *name)
        .collect();

    let mut modules : Vec<Module> = module_infos.iter()
        .map(|(name, module_type, outputs)| {
            Module {
                name: names.iter().position(|nm| nm == name).unwrap(),
                inputs: vec![],
                outputs: outputs.iter()
                    .map(|out| match names.iter().position(|nm| nm == out) {
                        Some(i) => i,
                        None => usize::MAX,
                    })
                    .collect(),
                module_type: module_type.clone(),
            }
        })
        .collect();


    // MAP INPUTS
    
    let mut inputs : Vec<Vec<usize>> = modules.iter().map(|_| vec![]).collect();

    for module in modules.iter() {
        for output in module.outputs.iter() {
            // untyped
            if *output == usize::MAX {
                continue;
            }

            inputs[*output].push(module.name);
        }
    }

    for module in modules.iter_mut() {
        module.inputs = inputs[module.name].clone();
        module.initialize();
    }

    part1(&names, MachineState { modules: modules.clone() });
    part2(&names, &mut MachineState { modules: modules.clone() });
}

fn part1(names : &Vec<&str>, mut machine : MachineState)
{
    let mut lows_sum = 0;
    let mut highs_sum = 0;

    for _ in 0..1000 {
        let (lows, highs) = send_pulse(names, &mut machine);
        lows_sum += lows;
        highs_sum += highs;
    }

    println!("PART 1: {}", lows_sum * highs_sum);
}

fn part2(names : &Vec<&str>, mut machine : &mut MachineState) {
    let mut i = 0;

    loop {
        i += 1;

        let mut pulses : Vec<PulseMessage> = vec![PulseMessage { 
            pulse: Pulse::Low, 
            sender: usize::MAX, 
            receiver: names.iter().position(|name| *name == "broadcaster").unwrap(),
        }];

        while pulses.len() > 0 {
            let msg = pulses.remove(0);
    
            // Terminal
            if msg.receiver == usize::MAX {
                continue;
            }
    
            if names[msg.receiver] == "rx" && msg.pulse == Pulse::Low {
                println!("PART 2: {}", i);
                return;
            }
    
            let receiver = &mut machine.modules[msg.receiver];
            let msgs = receiver.receive_pulse(msg);
            
            for msg in msgs {
                pulses.push(msg);
            }
        }
    }
}

fn name<'a>(names : &Vec<&'a str>, index : usize) -> &'a str {
    if index == usize::MAX {
        "UNKNOWN"
    } else {
        names[index]
    }
}

fn send_pulse(names : &Vec<&str>, mut machine : &mut MachineState) -> (usize, usize) {
    let mut pulses : Vec<PulseMessage> = vec![PulseMessage { 
        pulse: Pulse::Low, 
        sender: usize::MAX, 
        receiver: names.iter().position(|name| *name == "broadcaster").unwrap(),
    }];

    let mut lows = 0;
    let mut highs = 0;

    while pulses.len() > 0 {
        let msg = pulses.remove(0);

        // println!("{} sends {} to {}", name(names, msg.sender), match msg.pulse {
        //     Pulse::High => "HIGH",
        //     Pulse::Low => "LOW",
        // }, name(names, msg.receiver));
        
        match msg.pulse {
            Pulse::High => highs += 1,
            Pulse::Low => lows += 1,
        };

        // Terminal
        if msg.receiver == usize::MAX {
            continue;
        }

        let receiver = &mut machine.modules[msg.receiver];
        let msgs = receiver.receive_pulse(msg);
        
        for msg in msgs {
            pulses.push(msg);
        }
    }

    return (lows, highs)
}

#[derive(Clone, Copy)]
struct PulseMessage {
    pulse : Pulse,
    sender : usize,
    receiver : usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    High,
    Low
}

#[derive(Clone, PartialEq, Eq)]
struct MachineState {
    modules : Vec<Module>,
}

#[derive(Clone, PartialEq, Eq)]
struct Module {
    name : usize,
    inputs : Vec<usize>,
    outputs : Vec<usize>,
    module_type : ModuleType,
}

impl Module {
    fn initialize(&mut self) {
        if let ModuleType::Conjuction(conj) = &mut self.module_type {
            conj.initialize(&self.inputs);
        }
    }

    fn receive_pulse(&mut self, msg : PulseMessage) -> Vec<PulseMessage> {
        let mut msgs = vec![];

        match &mut self.module_type {
            ModuleType::FlipFlop(flip_flop) => {
                match msg.pulse {
                    Pulse::High => {}, // ignore
                    Pulse::Low => {
                        flip_flop.is_on = !flip_flop.is_on;

                        let pulse = match flip_flop.is_on {
                            true => Pulse::High,
                            false => Pulse::Low,
                        };

                        msgs.append(&mut self.msg_for_all_outputs(pulse));
                    },
                }
            },
            ModuleType::Conjuction(conj) => {
                conj.remember(msg);

                let pulse = match conj.all_high() {
                    true => Pulse::Low,
                    false => Pulse::High,
                };

                msgs.append(&mut self.msg_for_all_outputs(pulse));
            },
            ModuleType::Broadcast => {
                // Relay pulse directly 
                msgs.append(&mut self.msg_for_all_outputs(msg.pulse))
            }
        }

        msgs
    }

    fn msg_for_all_outputs(&self, pulse : Pulse) -> Vec<PulseMessage> {
        self.outputs.iter()
            .map(|output| PulseMessage{
                pulse,
                sender: self.name.clone(),
                receiver: *output,
            })
            .collect()
    }
}

#[derive(Clone, PartialEq, Eq)]
enum ModuleType {
    FlipFlop(FlipFlop),
    Conjuction(Conjuction),
    Broadcast,
}

#[derive(Clone, PartialEq, Eq)]
struct FlipFlop {
    is_on : bool
}

#[derive(Clone, PartialEq, Eq)]
struct Conjuction {
    last_inputs : HashMap<usize, Pulse>
}

impl Conjuction {
    fn initialize(&mut self, inputs : &Vec<usize>) {
        for key in inputs.iter() {
            self.last_inputs.insert(*key, Pulse::Low);
        }
    }

    fn remember(&mut self, msg : PulseMessage) {
        self.last_inputs.insert(msg.sender, msg.pulse);
    }

    fn all_high(&self) -> bool {
        self.last_inputs.values()
            .all(|val| *val == Pulse::High)
    }
}