// I didnt find out the trick here... 
// part one could be done way faster
// part two runs forever

// looked up a better approach for part two online...

use std::{fmt, result};
use utils;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use tokio::sync::{mpsc, oneshot};
use tokio::task;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use tokio::time::{timeout, Duration};

#[derive(Debug, Clone, PartialEq, Default)]
struct Button {
    target_leds: Vec<bool>,
    wirings: Vec<Vec<i32>>,
    joltage: Vec<i32>,
    hash_vault: HashMap<u64, usize>,
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format target_leds as . and #
        let leds_str: String = self
            .target_leds
            .iter()
            .map(|&b| if b { '#' } else { '.' })
            .collect();

        // Format wirings as "[1,2,3]; [4,5]"
        let wirings_str = self
            .wirings
            .iter()
            .map(|w| {
                let nums = w
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                format!("[{}]", nums)
            })
            .collect::<Vec<_>>()
            .join("; ");

        // Format joltage as "1,2,3"
        let joltage_str = self
            .joltage
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");

        write!(
            f,
            "Button {{ target_leds: {}, wirings: {}, joltage: {} }}",
            leds_str, wirings_str, joltage_str
        )
    }
}

fn same_multiplier(a: &[i32], b: &[i32]) -> Option<i32> {
    if a.len() != b.len() || a.is_empty() {
        return None;
    }

    let (num, den) = (b[0], a[0]); // factor = num / den

    if den == 0 {
        return None;
    }

    // all indices must satisfy: bi * den == ai * num
    let ok = a.iter()
        .zip(b.iter())
        .all(|(&ai, &bi)| bi * den == ai * num);

    if !ok {
        return None;
    }

    // ensure factor is an integer
    if num % den != 0 {
        return None;
    }

    Some(num / den)
}

//https://github.com/JoanaBLate/advent-of-code-js/blob/main/2025/day10-solve2.js
#[derive(Debug, Clone)]
struct ComboObj {
    presses: usize,
    joltage: Vec<i32>,
    button_indices: Vec<usize>,
}

fn fill_patterns_and_combos(buttons: &Vec<Vec<i32>>, joltages_length: usize) -> HashMap<String, Vec<ComboObj>> {
    let mut combos_by_pattern: HashMap<String, Vec<ComboObj>> = HashMap::new();
    
    let off = 2_usize.pow(buttons.len() as u32);
    
    for n in 0..off {
        fill_patterns_and_combos_with(n, buttons, joltages_length, &mut combos_by_pattern);
    }
    
    combos_by_pattern
}

fn fill_patterns_and_combos_with(
    bitwise_combo: usize, 
    buttons: &Vec<Vec<i32>>, 
    joltages_length: usize,
    combos_by_pattern: &mut HashMap<String, Vec<ComboObj>>
) {
    let mut presses = 0;
    let mut joltage = vec![0; joltages_length];
    let mut button_indices = Vec::new();
    
    for button_index in 0..buttons.len() {
        let adjusted_button_index = buttons.len() - 1 - button_index;
        
        let button_value_in_bitwise_combo = bitwise_combo >> adjusted_button_index;
        
        if (button_value_in_bitwise_combo & 1) != 1 {
            continue;
        }
        
        presses += 1;
        button_indices.push(button_index); // Speichere welcher Button
        
        let button = &buttons[button_index];
        
        for &machine_index in button {
            joltage[machine_index as usize] += 1;
        }
    }
    
    let pattern = pattern_from_joltage(&joltage);
    
    combos_by_pattern
        .entry(pattern)
        .or_insert_with(Vec::new)
        .push(ComboObj { 
            presses, 
            joltage,
            button_indices 
        });
}

fn pattern_from_joltage(joltage: &Vec<i32>) -> String {
    joltage.iter()
        .map(|&jolt| ((jolt % 2) as u8 + b'0') as char)
        .collect()
}

impl Button {
    pub fn solve_joltage_optimized(&mut self) -> Option<Vec<i32>> {
        let combos_by_pattern = fill_patterns_and_combos(&self.wirings, self.joltage.len());
        
        let mut cache: HashMap<String, Option<(usize, Vec<i32>)>> = HashMap::new();
        
        if let Some((total_presses, steps)) = self.count_presses(
            &self.joltage.clone(), 
            &combos_by_pattern, 
            &mut cache
        ) {
            println!("Total presses: {}, Steps: {:?}", total_presses, steps);
            Some(steps)
        } else {
            None
        }
    }
    
    fn count_presses(
        &self,
        target: &Vec<i32>,
        combos_by_pattern: &HashMap<String, Vec<ComboObj>>,
        cache: &mut HashMap<String, Option<(usize, Vec<i32>)>>
    ) -> Option<(usize, Vec<i32>)> {
        let key = target.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
        
        if let Some(cached) = cache.get(&key) {
            return cached.clone();
        }
        
        // Prüfe ob nur Nullen
        let only_zeros = target.iter().all(|&jolt| jolt == 0);
        if only_zeros {
            let empty_steps = vec![0; self.wirings.len()];
            let result = Some((0, empty_steps));
            cache.insert(key, result.clone());
            return result;
        }
        
        // Prüfe auf negative Werte
        if target.iter().any(|&jolt| jolt < 0) {
            cache.insert(key, None);
            return None;
        }
        
        let pattern = pattern_from_joltage(target);
        
        let combos = match combos_by_pattern.get(&pattern) {
            Some(c) => c,
            None => {
                cache.insert(key, None);
                return None;
            }
        };
        
        let mut best_result: Option<(usize, Vec<i32>)> = None;
        
        for combo_obj in combos {
            let mut half_target = vec![0; self.joltage.len()];
            
            let mut valid = true;
            for joltage_index in 0..self.joltage.len() {
                let new_jolt = target[joltage_index] - combo_obj.joltage[joltage_index];
                
                if new_jolt < 0 || new_jolt % 2 != 0 {
                    valid = false;
                    break;
                }
                
                half_target[joltage_index] = new_jolt / 2;
            }
            
            if !valid {
                continue;
            }
            
            if let Some((sub_presses, sub_steps)) = self.count_presses(
                &half_target, 
                combos_by_pattern, 
                cache
            ) {
                let total_presses = combo_obj.presses + 2 * sub_presses;
                
                // Baue die Steps auf
                let mut steps = vec![0; self.wirings.len()];
                
                // Füge die Combo-Buttons hinzu (einmal)
                for &button_idx in &combo_obj.button_indices {
                    steps[button_idx] += 1;
                }
                
                // Füge die Sub-Steps hinzu (doppelt, wegen Halbierung)
                for i in 0..steps.len() {
                    steps[i] += 2 * sub_steps[i];
                }
                
                // Behalte die beste Lösung
                if best_result.is_none() || total_presses < best_result.as_ref().unwrap().0 {
                    best_result = Some((total_presses, steps));
                }
            }
        }
        
        cache.insert(key, best_result.clone());
        best_result
    }

    pub async fn solve_joltage_parallel(&mut self, worker_count: usize) -> Option<Vec<i32>> {
        type Params = (Vec<i32>, Vec<i32>);
        let wirings = Arc::new(self.wirings.clone());
        let joltage = Arc::new(self.joltage.clone());
        let hash_vault = Arc::new(Mutex::new(std::mem::take(&mut self.hash_vault)));
    
        let (job_tx, job_rx) = mpsc::channel::<Params>(100000);
        let (hit_tx, mut hit_rx) = mpsc::channel::<Params>(1);
        let found = Arc::new(AtomicBool::new(false));
        let job_rx = Arc::new(Mutex::new(job_rx));
    
        //backup stop 
        let expanded = Arc::new(AtomicUsize::new(0));
        let max_expanded = 10000000;
    
        let mut worker_handles = Vec::new();
    
        for _ in 0..worker_count {
            let job_rx = job_rx.clone();
            let hit_tx = hit_tx.clone();
            let job_tx_for_worker = job_tx.clone();
            let found = found.clone();
            let expanded = expanded.clone();
    
            let wirings = wirings.clone();
            let joltage = joltage.clone();
            let hash_vault = hash_vault.clone();
    
            let handle = task::spawn(async move {
                loop {
                    let params_opt = {
                        let mut guard = job_rx.lock().await;
                        let res = timeout(Duration::from_secs(1), guard.recv()).await;
                        match res {
                            Ok(msg_opt) => msg_opt,        // normales recv-Ergebnis (Some/None)
                            Err(_) => {
                                // Timeout abgelaufen
                                //println!("recv timeout, breche Worker ab");
                                found.store(true, Ordering::Relaxed);
                                return; // Worker-Task endet, Sender-/Receiver-Clone werden gedroppt
                            }
                        }
                    };
    
                    let (current, mut steps) = match params_opt {
                        Some(p) => p,
                        None => break,
                    };
                    
                    if found.load(Ordering::Relaxed) {
                        break;
                    }
    
                    let check = {
                        let mut hv = hash_vault.lock().await;
                        Button::check_joltage_static(&current, &mut steps, &joltage, &mut hv)
                    };
    
                    if check < 0 { continue; }
                    else if check == 1 {
                        if !found.swap(true, Ordering::Relaxed) {
                            let hit: Params = (current, steps);
                            let _ = hit_tx.send(hit).await;
                            println!("found a result");
                        }
                        break;
                    } else {
                        if found.load(Ordering::Relaxed) {
                            break;
                        }
                        
                        let job_tx_for_spawn = job_tx_for_worker.clone();
                        let next = Button::get_next_steps_smart(&wirings, &current, &steps, &joltage);
    
                        tokio::spawn(async move {
                            for p in next {
                                if job_tx_for_spawn.send(p).await.is_err() {
                                    break;
                                }
                            }
                        });
                    }
                }
            });
            
            worker_handles.push(handle);
        }
    
        drop(job_rx);
        drop(hit_tx);
    
        let curr: Vec<i32> = vec![0; joltage.len()];
        let steps: Vec<i32> = vec![0; wirings.len()];
    
        for params in Button::get_next_steps_smart(&wirings, &curr, &steps, &joltage) {
            job_tx.send(params).await.unwrap();
        }
    
        drop(job_tx);
        
        let result = hit_rx.recv().await;
        
        // Cleanup: abort all workers to stop immediately
        for handle in worker_handles {
            handle.abort();
        }
        
        if let Some(hit) = result {
            Some(hit.1)
        } else {
            println!("Kein Treffer gefunden.");
            None
        }
    }

    fn check_joltage_static(current: &Vec<i32>, steps: &mut Vec<i32>, joltage: &Vec<i32>, hash_vault: &mut HashMap<u64, usize>) -> i32 {
        let mut s = DefaultHasher::new();
        current.hash(&mut s);
        let hash = s.finish();

        let sum: usize = steps.iter().sum::<i32>() as usize;
        if let Some(&best_len) = hash_vault.get(&hash) {
            //println!("skipped {:?}", current);
            if best_len <= sum {
                return -1;
            }
        }

        // if any of current is already > joltage target
        if current.iter().zip(joltage.iter()).any(|(c, t)| c > t) {
            return -1;
        }

        /*let a = same_multiplier(&current, &joltage);
        if let Some(b) = a {
            println!("early multi found");
            for i in steps {
                *i *= b;
            }
        }*/

        hash_vault.insert(hash, sum);

        //println!("{:?}, {:?} = {}", current, joltage, current == joltage);
        if current == joltage {
            //println!("succes with steps: {:?}", steps);
            1
        } else {
            0
        }
    }

    fn quick_check(current: &Vec<i32>, joltage: &Vec<i32>) -> bool {
        return current.iter().zip(joltage.iter()).any(|(c, t)| c > t);
    }

    fn get_next_steps_smart(wirings: &Vec<Vec<i32>>, current: &Vec<i32>, steps: &Vec<i32>, joltage: &Vec<i32>) -> Vec<(Vec<i32>, Vec<i32>)> {
        let mut result = Vec::new();

        //rest = target - current
        let rest: Vec<i32> = joltage.iter()
        .zip(current.iter())
        .map(|(x, y)| x - y)
        .collect();

        //min of rest except the 0s
        let min_index = rest
        .iter()
        .enumerate()
        .filter(|&(_, value)| *value > 0)
        .min_by_key(|&(_, value)| value)
        .map(|(idx, _)| idx as i32);

        if let Some(minimum) = min_index {
            //add all new paths that lead to min going to 0, ignore rest of wirings
            //add all paths until we 
            for(i, wiring) in wirings.iter().enumerate() {
                //only wirings that affect index of smallest rest number
                if wiring.contains(&minimum) {
                    let mut new_steps = steps.clone();
                    new_steps[i] += 1;

                    let mut new_curr = current.clone();
                    for w in wiring {
                        new_curr[*w as usize] += 1;
                    }

                    if new_curr.iter().zip(joltage.iter()).any(|(c, t)| c > t) {
                        continue;
                    }

                    result.push((new_curr, new_steps));
                }
            }
        }
        return result;
    }

    // recursive depth search
    fn solve_leds(&mut self) -> Vec<usize> {
        let curr: Vec<bool> = vec![false; self.target_leds.len()];
        if let Some(result) = self._solve_leds(curr, vec![]) {
            return result;
        }
        return vec![];
    }

    fn _solve_leds(&mut self, current: Vec<bool> , old_steps: Vec<usize>) -> Option<Vec<usize>> {
        let mut results: Vec<Vec<usize>> = Vec::new();
        
        for (i, wiring) in self.wirings.clone().iter().enumerate() {
            let mut steps = old_steps.clone();
            steps.push(i);

            let mut curr = current.clone();
            for w in wiring {
                curr[*w as usize] = !curr[*w as usize];
            }
            
            let mut s = DefaultHasher::new();
            curr.hash(&mut s);
            let hash = s.finish();

            if let Some(&best_len) = self.hash_vault.get(&hash) {
                if best_len <= steps.len() {
                    continue;
                }
            }

            self.hash_vault.insert(hash, steps.len());

            if curr == self.target_leds {
                results.push(steps.clone());
            } else {
                let res = self._solve_leds(curr, steps.clone());
                if let Some(r) = res {
                    results.push(r);
                }
            }
        }

        results.sort_by_key(|s| s.len());
        if results.len() > 0 {
            return Some(results[0].clone());
        }
        None
    }
}

#[tokio::main]
async fn main() {
    if let Ok(lines) = utils::read_lines("./input.txt") {
        let mut buttons: Vec<Button> = Vec::new();

        for (i, mut line) in lines.enumerate() {
            if let Ok(ref mut l) = line {
                let mut b: Button = Button::default();
                let mut a: Vec<&str> = l.split_ascii_whitespace().collect();
                b.target_leds = a
                    .remove(0)
                    .chars()
                    .filter_map(|c| {
                        if c == '.' {
                            Some(false)
                        } else if c == '#' {
                            Some(true)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<bool>>();

                b.joltage = a
                    .remove(a.len() - 1)
                    .split(',')
                    .filter_map(|part| {
                        let cleaned = part.replace('{', "").replace('}', "");
                        if let Ok(l) = cleaned.parse::<i32>() {
                            Some(l)
                        } else {
                            None
                        }
                    })
                    .collect();

                for bracket in a {
                    let nums: Vec<i32> = bracket
                        .split(',')
                        .filter_map(|part| {
                            let cleaned = part.replace('(', "").replace(')', "");
                            if let Ok(num) = cleaned.parse::<i32>() {
                                Some(num)
                            } else {
                                None
                            }
                        })
                        .collect();
                    b.wirings.push(nums);
                }
                buttons.push(b);
            }
        }
        println!("{:?}", buttons);

        let result1 = one(buttons.clone());
        let result2 = two_2(buttons.clone());
        
        println!("result 1: {} 2: {}", result1, result2);
    }
}

fn one(buttons:  Vec<Button>) -> usize {
    let mut result = 0;
    for mut button in buttons {
        let steps = button.solve_leds();
        println!("{} {:?} {}", button, steps, steps.len());
        result += steps.len();
    }
    result
}

async fn two(mut buttons: Vec<Button>) -> i32 {
    let worker_count = 16;
    let mut result = 0;

    let mut ind = 0;

    for button in buttons.iter_mut() {
        println!("{ind} {:?}", button);
        if let Some(steps) = button.solve_joltage_parallel(worker_count).await {
            let min_steps = steps.iter().sum::<i32>() as i32;
            result += min_steps;
            println!("{ind} {:?} = {} with {:?}", button, min_steps, steps);

            let mut check = vec![0; button.joltage.len()];

            for (i, count) in steps.iter().enumerate() {
                let wiring = &button.wirings[i];
                for w in wiring {
                    check[*w as usize] += count;
                }
            }
            println!("{:?} == {:?} = {}", button.joltage, check, button.joltage == check);
        } else {
            println!("shiat");
        }
        ind+=1;
    }

    result
}

fn two_2(mut buttons: Vec<Button>) -> i32  {
    let mut result = 0;
    for button in &mut buttons {
        println!("Button: {:?}" , button);
        let steps = button.solve_joltage_optimized().unwrap();
        let min_steps = steps.iter().sum::<i32>() as i32;
        println!("solved with {} steps: {:?}", min_steps, steps);

        let mut check = vec![0; button.joltage.len()];

        for (i, count) in steps.iter().enumerate() {
            let wiring = &button.wirings[i];
            for w in wiring {
                check[*w as usize] += count;
            }
        }
        println!("{:?} == {:?} = {}", button.joltage, check, button.joltage == check);
        result += min_steps;
    }
    return result;
}