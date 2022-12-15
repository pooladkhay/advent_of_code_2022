use std::{collections::VecDeque, fs};

enum Instruction {
    // u8 represents number of cycles required for a given instruction
    // i64 is the value to add
    Addx(u8, i64),
    Noop(u8),
}

struct Cpu<'a> {
    rx: i64,
    cycle_count: i64,
    signal_strength_cycles: VecDeque<i64>,
    crt_current: Vec<&'a str>,
    crt_all: Vec<Vec<&'a str>>,
}

impl Cpu<'_> {
    fn draw_pixel(&mut self) {
        let mut cycle_count = 0;
        match self.cycle_count {
            1..=39 => cycle_count = self.cycle_count,
            40..=79 => cycle_count = self.cycle_count - 40,
            80..=119 => cycle_count = self.cycle_count - 80,
            120..=159 => cycle_count = self.cycle_count - 120,
            160..=199 => cycle_count = self.cycle_count - 160,
            200..=239 => cycle_count = self.cycle_count - 200,
            _ => (),
        }

        if cycle_count == self.rx - 1 || cycle_count == self.rx || cycle_count == self.rx + 1 {
            self.crt_current.push("#")
        } else {
            self.crt_current.push(".")
        }

        if self.crt_current.len() == 40 {
            self.crt_all.push(self.crt_current.clone());
            self.crt_current = Vec::with_capacity(40)
        }
    }

    fn calc_sig_strength(&mut self) -> i64 {
        self.signal_strength_cycles.pop_front();
        self.rx * self.cycle_count
    }

    // runs an opcode and returns signal strength
    fn run_op(&mut self, inst: Instruction) -> i64 {
        match inst {
            Instruction::Addx(c, v) => {
                let mut signal = 0_i64;
                for _ in 0..c {
                    if self.signal_strength_cycles.contains(&self.cycle_count) {
                        signal = self.calc_sig_strength();
                    }

                    self.draw_pixel();

                    self.cycle_count += 1;

                    if self.signal_strength_cycles.contains(&self.cycle_count) {
                        signal = self.calc_sig_strength();
                    }
                }
                self.rx += v;
                signal
            }
            Instruction::Noop(c) => {
                let mut signal = 0_i64;
                for _ in 0..c {
                    if self.signal_strength_cycles.contains(&self.cycle_count) {
                        signal = self.calc_sig_strength();
                    }

                    self.draw_pixel();

                    self.cycle_count += 1;

                    if self.signal_strength_cycles.contains(&self.cycle_count) {
                        signal = self.calc_sig_strength();
                    }
                }
                signal
            }
        }
    }
}

fn main() {
    let input = get_input("src/input.txt");

    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| {
            let line = line.trim();
            let line = line.split_whitespace().collect::<Vec<&str>>();
            if line.len() == 1 {
                Instruction::Noop(1)
            } else {
                Instruction::Addx(2, line.last().unwrap().parse::<i64>().unwrap())
            }
        })
        .collect();

    let mut cpu = Cpu {
        rx: 1,
        cycle_count: 0,
        signal_strength_cycles: VecDeque::from_iter(vec![20, 60, 100, 140, 180, 220]),
        crt_current: Vec::with_capacity(40),
        crt_all: Vec::with_capacity(6),
    };

    let mut sum_signal_strengths = 0_i64;

    for inst in instructions {
        sum_signal_strengths += cpu.run_op(inst);
    }

    println!("sum signal strengths: {}", sum_signal_strengths);
    println!();
    for l in cpu.crt_all {
        println!("{}", l.concat())
    }
}

fn get_input(path: &str) -> String {
    let input = fs::read_to_string(path);
    match input {
        Ok(input) => input,
        Err(err) => panic!("err reading input: {}", err),
    }
}
