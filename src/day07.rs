use permutohedron::Heap;
use crate::intcode::{Word, Emulator};

fn amp_chain(base: &Emulator, phases: &[Word]) -> Word {
    let mut signal: Word = 0;
    for phase in phases {
        let mut amp = base.clone();
        amp.write(*phase);
        amp.write(signal);
        amp.run();
        signal = *amp.read_all().last().unwrap();
    }
    return signal;
}

fn run_amp_chains(base: &Emulator) -> Word {
    let mut phases: Vec<Word> = (0 .. 5).collect();
    let heap = Heap::new(&mut phases);
    heap.map(|phases| amp_chain(base, phases.as_slice())).max().unwrap()
}

fn amp_feedback_loop(base: &Emulator, phases: &[Word]) -> Word {
    let mut amps: Vec<Emulator> =
        phases
        .iter()
        .map(|phase| {
            let mut amp = base.clone();
            amp.write(*phase);
            return amp;
        })
        .collect();
    let mut thruster_signal: Word = 0;
    let mut signal: Word = 0;
    'outer: loop {
        for amp in amps.iter_mut() {
            amp.write(signal);
            amp.run();      // Until halts or waits on new input
            if let Some(v) = amp.read() {
                signal = v;
            } else {
                // No output means it's halted and we read its last output already
                break 'outer;
            }
        }
        thruster_signal = signal;
    }
    thruster_signal
}

fn run_amp_feedback_loops(base: &Emulator) -> Word {
    let mut phases: Vec<Word> = (5 .. 10).collect();
    let heap = Heap::new(&mut phases);
    heap.map(|phases| amp_feedback_loop(base, phases.as_slice())).max().unwrap()
}

pub fn part1() -> i32 {
    run_amp_chains(&Emulator::from_data_file("day07_input.txt"))
}

pub fn part2() -> i32 {
    run_amp_feedback_loops(&Emulator::from_data_file("day07_input.txt"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_amp_chains_1() {
        let base = Emulator::from_data_file("day07_example1.txt");
        assert_eq!(run_amp_chains(&base), 43210);
    }

    #[test]
    fn test_run_amp_chains_2() {
        let base = Emulator::from_data_file("day07_example2.txt");
        assert_eq!(run_amp_chains(&base), 54321);
    }

    #[test]
    fn test_run_amp_chains_3() {
        let base = Emulator::from_data_file("day07_example3.txt");
        assert_eq!(run_amp_chains(&base), 65210);
    }

    #[test]
    fn test_run_amp_feedback_loops_1() {
        let base = Emulator::from_data_file("day07_example4.txt");
        assert_eq!(amp_feedback_loop(&base, &[9, 8, 7, 6, 5]), 139629729);
        assert_eq!(run_amp_feedback_loops(&base), 139629729);
    }

    #[test]
    fn test_run_amp_feedback_loops_2() {
        let base = Emulator::from_data_file("day07_example5.txt");
        assert_eq!(amp_feedback_loop(&base, &[9, 7, 8, 5, 6]), 18216);
        assert_eq!(run_amp_feedback_loops(&base), 18216);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 46248);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 54163586);
    }
}
