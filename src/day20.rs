use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    fs,
    process::exit,
};

#[derive(Clone)]
enum Module {
    FlipFlop(FlipFlopStruct),
    Conjunction(ConjunctionStruct),
    Debug(DebugStruct),
}

enum ModuleType {
    FlipFlop,
    Conjunction,
    Debug,
}

#[derive(Clone, Debug)]
struct FlipFlopStruct {
    state: bool,
    targets: Vec<String>,
}

#[derive(Clone, Debug)]
struct ConjunctionStruct {
    states: HashMap<String, bool>,
    targets: Vec<String>,
}

#[derive(Clone, Debug)]
struct DebugStruct {}

fn handle(
    node: String,
    signal: bool,
    source: String,
    modules: &mut HashMap<String, Module>,
    queue: &mut VecDeque<(String, bool, String)>,
    lows: &mut i64,
    highs: &mut i64,
) -> Option<bool> {
    // println!(" {} -{}-> {}", source, signal, node);
    // if node == "rx" {
    if node == "tj" {
        // println!("recieved");
        return Some(signal);
    }
    let module_type = match modules.get(&node).unwrap() {
        Module::FlipFlop(_) => ModuleType::FlipFlop,
        Module::Conjunction(_) => ModuleType::Conjunction,
        Module::Debug(_) => ModuleType::Debug,
    };

    let (targets, send_val) = match modules.get_mut(&node).unwrap() {
        Module::FlipFlop(data) => {
            if !signal {
                data.state = !data.state;
                (data.targets.clone(), data.state)
            } else {
                (Vec::new(), false)
            }
        }
        Module::Conjunction(data) => {
            data.states.insert(source.clone(), signal);
            if data.states.iter().all(|(_k, v)| *v) {
                (data.targets.clone(), false)
            } else {
                (data.targets.clone(), true)
            }
        }
        Module::Debug(_data) => {
            // println!(" {} got {} from {}", node, signal, source);
            (Vec::new(), false)
        }
    };

    match module_type {
        ModuleType::FlipFlop => {
            for target in targets {
                if send_val {
                    *highs += 1;
                } else {
                    *lows += 1;
                }
                queue.push_back((target, send_val, node.clone()));
                // handle(target, send_val, &node, modules, lows, highs);
            }
        }
        ModuleType::Conjunction => {
            if send_val {
                for target in targets {
                    *highs += 1;
                    queue.push_back((target, true, node.clone()));
                    // handle(target.clone(), true, &node, modules, lows, highs);
                }
            } else {
                for target in targets {
                    *lows += 1;
                    queue.push_back((target, false, node.clone()));
                    // handle(target.clone(), false, &node, modules, lows, highs);
                }
            }
        }
        ModuleType::Debug => {}
    }
    return None;
}

pub fn part_a() {
    if let Ok(data) = fs::read_to_string("day20.txt") {
        let mut modules = HashMap::new();
        modules.insert("output".to_string(), Module::Debug(DebugStruct {}));
        let mut broadcasts = Vec::new();
        for line in data.lines() {
            let mut parts = line.split(" -> ");
            let name = parts.next().unwrap();
            if name != "broadcaster" {
                match &name[0..1] {
                    "%" => {
                        modules.insert(
                            (name[1..]).to_string(),
                            Module::FlipFlop(FlipFlopStruct {
                                state: false,
                                targets: Vec::new(),
                            }),
                        );
                    }
                    "&" => {
                        modules.insert(
                            name[1..].to_string(),
                            Module::Conjunction(ConjunctionStruct {
                                states: HashMap::new(),
                                targets: Vec::new(),
                            }),
                        );
                    }
                    _ => {
                        panic!("invalid input");
                    }
                }
            }
        }
        for line in data.lines() {
            let mut parts = line.split(" -> ");
            let name = parts.next().unwrap();
            let dests = parts.next().unwrap().split(", ");
            if name == "broadcaster" {
                for dest in dests {
                    broadcasts.push(dest.to_string());
                    if let Module::Conjunction(target) = modules.get_mut(dest).unwrap() {
                        target.states.insert(name[1..].to_string(), false);
                    }
                }
            } else {
                match &name[0..1] {
                    "%" => {
                        for dest in dests {
                            if let Module::FlipFlop(module) = modules.get_mut(&name[1..]).unwrap() {
                                module.targets.push(dest.to_string());
                                if let Module::Conjunction(target) = modules.get_mut(dest).unwrap()
                                {
                                    target.states.insert(name[1..].to_string(), false);
                                }
                            }
                        }
                    }
                    "&" => {
                        for dest in dests {
                            if let Module::Conjunction(module) =
                                modules.get_mut(&name[1..]).unwrap()
                            {
                                module.targets.push(dest.to_string());
                                // println!(" {}", dest);
                                if let Some(Module::Conjunction(target)) = modules.get_mut(dest) {
                                    target.states.insert(name[1..].to_string(), false);
                                }
                            }
                        }
                    }
                    _ => {
                        panic!("invalid input");
                    }
                }
            }
        }

        let mut lows = 0;
        let mut highs = 0;
        for _ in 0..1000 {
            lows += 1;
            let mut queue = VecDeque::new();
            for target in &broadcasts {
                lows += 1;
                queue.push_back((target.clone(), false, "broadcaster".to_string()));
            }
            while let Some((node, signal, source)) = queue.pop_front() {
                handle(
                    node,
                    signal,
                    source,
                    &mut modules,
                    &mut queue,
                    &mut lows,
                    &mut highs,
                );
            }
            // println!(" -");
        }

        println!("{} {} ({})", lows, highs, lows * highs);
    }
}

pub fn part_b() {
    if let Ok(data) = fs::read_to_string("day20.txt") {
        let mut modules = HashMap::new();
        modules.insert("output".to_string(), Module::Debug(DebugStruct {}));
        modules.insert("rx".to_string(), Module::Debug(DebugStruct {}));
        let mut broadcasts = Vec::new();

        for line in data.lines() {
            let mut parts = line.split(" -> ");
            let name = parts.next().unwrap();
            if name != "broadcaster" {
                match &name[0..1] {
                    "%" => {
                        modules.insert(
                            (name[1..]).to_string(),
                            Module::FlipFlop(FlipFlopStruct {
                                state: false,
                                targets: Vec::new(),
                            }),
                        );
                    }
                    "&" => {
                        modules.insert(
                            name[1..].to_string(),
                            Module::Conjunction(ConjunctionStruct {
                                states: HashMap::new(),
                                targets: Vec::new(),
                            }),
                        );
                    }
                    _ => {
                        panic!("invalid input");
                    }
                }
            }
        }

        for line in data.lines() {
            let mut parts = line.split(" -> ");
            let name = parts.next().unwrap();
            let dests = parts.next().unwrap().split(", ");
            if name == "broadcaster" {
                for dest in dests {
                    broadcasts.push(dest.to_string());
                    if let Module::Conjunction(target) = modules.get_mut(dest).unwrap() {
                        target.states.insert(name[1..].to_string(), false);
                    }
                }
            } else {
                match &name[0..1] {
                    "%" => {
                        for dest in dests {
                            if let Module::FlipFlop(module) = modules.get_mut(&name[1..]).unwrap() {
                                module.targets.push(dest.to_string());
                                if let Module::Conjunction(target) = modules.get_mut(dest).unwrap()
                                {
                                    target.states.insert(name[1..].to_string(), false);
                                }
                            }
                        }
                    }
                    "&" => {
                        for dest in dests {
                            if let Module::Conjunction(module) =
                                modules.get_mut(&name[1..]).unwrap()
                            {
                                module.targets.push(dest.to_string());
                                // println!(" {}", dest);
                                if let Some(Module::Conjunction(target)) = modules.get_mut(dest) {
                                    target.states.insert(name[1..].to_string(), false);
                                }
                            }
                        }
                    }
                    _ => {
                        panic!("invalid input");
                    }
                }
            }
        }

        // let limb = get_subchain("gh".to_string(), &mut modules);

        let mut loop_sizes = Vec::new();
        for broadcast_target in broadcasts {
            let mut lows = 0;
            let mut highs = 0;
            let mut cycles = 0;
            'find_cycle: for _ in 0..13000 {
                cycles += 1;
                let mut queue = VecDeque::new();
                queue.push_back((broadcast_target.clone(), false, "broadcaster".to_string()));
                while let Some((node, signal, source)) = queue.pop_front() {
                    if let Some(result) = handle(
                        node,
                        signal,
                        source,
                        &mut modules,
                        &mut queue,
                        &mut lows,
                        &mut highs,
                    ) {
                        if result {
                            loop_sizes.push(cycles);
                            break 'find_cycle;
                            // println!(" {}: {} at {}", broadcast_target, result, cycles);
                        }
                    }
                }
            }
        }
        print!("{}", loop_sizes.iter().product::<i64>());
        // for (k, v) in limb {
        //     match v {
        //         Module::Conjunction(d) => {
        //             println!(" {} -> {:?}", k, d.targets)
        //         }
        //         Module::FlipFlop(d) => {
        //             println!(" {} -> {:?}", k, d.targets)
        //         }
        //         Module::Debug(_) => {}
        //     }
        // }

        // let mut count: i64 = 0;
        // let mut lows = 0;
        // let mut highs = 0;
        // 'outer: loop {
        //     if count > 3 {
        //         break;
        //     }
        //     count += 1;
        //     lows += 1;
        //     let mut queue = VecDeque::new();
        //     for target in &broadcasts {
        //         lows += 1;
        //         queue.push_back((target.clone(), false, "broadcaster".to_string()));
        //     }
        //     while let Some((node, signal, source)) = queue.pop_front() {
        //         if handle(
        //             node,
        //             signal,
        //             source,
        //             &mut modules,
        //             &mut queue,
        //             &mut lows,
        //             &mut highs,
        //         ) {
        //             break 'outer;
        //         }
        //     }
        //     println!(" -");
        // }

        // println!("{}", count);
    }
}

fn get_subchain(node: String, modules: &mut HashMap<String, Module>) -> HashMap<String, Module> {
    let mut result = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back(node);
    while let Some(cur) = queue.pop_front() {
        if cur == "tj" {
            continue;
        }
        let module = modules.get(&cur).unwrap();
        result.insert(cur.clone(), (*module).clone());
        match module {
            Module::FlipFlop(data) => {
                for val in &data.targets {
                    if !result.contains_key(val) {
                        queue.push_back(val.clone());
                    }
                }
            }
            Module::Conjunction(data) => {
                for val in &data.targets {
                    if !result.contains_key(val) {
                        queue.push_back(val.clone());
                    }
                }
            }
            Module::Debug(_) => {}
        }
    }

    return result;
}
