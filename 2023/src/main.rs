use core::str;
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;

#[allow(dead_code)]
fn aoc_print_vec(input: &Vec<i32>) {
    // do not delete this function, to no forget such silly behavior :D
    for v in input.into_iter() {
        print!("{} ", v);
    }
    println!("");
}

fn aoc_gcm(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    println!("AOC_LIB Checking: {} {}", a, b);
    if a > b {
        return aoc_gcm(b, a%b);
    } else {
        return aoc_gcm(a, b%a);
    }
}

fn aoc_lcm(a: u64, b: u64) -> u64 {
    return a*b / aoc_gcm(a, b);
}

fn day_1_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 1 part 1");
    let mut result = 0;

    for (_,line) in input_lines.into_iter().enumerate() {
        //println!("{}", line);
        let mut first_num = -1;
        let mut last_num = -1;
        for c in line.chars() {
            if c >= '0' && c <= '9' {
                last_num = c as i32 - '0' as i32;
                if first_num < 0 {
                    first_num = last_num;
                }
            }
        }
        //println!("First: {} Last: {}", first_num, last_num);
        result += 10*first_num + last_num;
    }

    println!("Final result: {}", result);
}

fn day_1_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 1 part 2");
    let mut result = 0;

    let string_numbers = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for (_,line) in input_lines.into_iter().enumerate() {
        //println!("{}", line);
        let mut first_num_idx = -1;
        let mut last_num_idx = -1;
        let mut first_num = 0;
        let mut last_num = 0;
        for idx in 0..line.chars().count() {
            let c = line.chars().nth(idx).unwrap();
            if c >= '0' && c <= '9' {
                last_num_idx = idx as i32;
                last_num = c as i32 - '0' as i32;
                if first_num_idx < 0 {
                    first_num_idx = idx as i32;
                    first_num = last_num;
                }
            }
        }

        for number in string_numbers.keys() {
            let tmp = line.find(number);
            if tmp.is_none() {
                continue;
            }
            let tmp = tmp.unwrap() as i32;
            if first_num_idx < 0 || tmp < first_num_idx {
                first_num_idx = tmp;
                first_num = string_numbers[number];
            }
            let tmp = line.rfind(number).unwrap() as i32;
            if last_num_idx < 0 || tmp > last_num_idx {
                last_num_idx = tmp;
                last_num = string_numbers[number];
            }
        }

        //println!("First: {} Last: {}", first_num, last_num);
        result += 10*first_num + last_num;
    }

    println!("Final result: {}", result);
}

fn day_2_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 2 part 1");
    let balls_limits = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
    let mut result = 0;

    for (_,line) in input_lines.into_iter().enumerate() {
        let mut idx = 5;
        let mut game_number = 0;
        for val in line.chars().skip(idx) {
            if val == ':' {
                break;
            }
            game_number = game_number*10 + (val as i32 - '0' as i32) as i32;
            idx += 1;
        }
        //println!("Parsing game number {}", game_number);
        //println!("\tInput: {}", line);
        let mut ball_cnt = 0;
        let mut color_idx = 0;
        let mut valid = true;
        let game = &line[idx+1..line.chars().count()];
        for (idx, val) in game.chars().enumerate() {
            //println!("Idx: {} val: {}", idx, val);
            if val == ' ' {
                continue;
            }
            if val == ';' || val == ',' || idx == game.chars().count()-1 {
                let ball_color = if idx == game.chars().count()-1 {
                    &game[color_idx..idx+1]
                } else {
                    &game[color_idx..idx]
                };
                //println!("Balls color: {} count: {}", &ball_color, ball_cnt);
                if ball_cnt > *balls_limits.get(ball_color).unwrap() {
                    valid = false;
                    break;
                }
                ball_cnt = 0;
                color_idx = 0;
            } else if val >= '0' && val <= '9' {
                ball_cnt = ball_cnt * 10 + (val as i32 - '0' as i32);
            } else if color_idx == 0 {
                color_idx = idx;
            }
        }
        if valid {
            result += game_number;
        }
    }

    println!("Final result: {}", result);
}

fn day_2_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 2 part 2");
    let mut result = 0;

    for (_,line) in input_lines.into_iter().enumerate() {
        let mut idx = 5;
        let mut game_number = 0;
        for val in line.chars().skip(idx) {
            if val == ':' {
                break;
            }
            game_number = game_number*10 + (val as i32 - '0' as i32) as i32;
            idx += 1;
        }
        //println!("Parsing game number {}", game_number);
        //println!("\tInput: {}", line);
        let mut ball_cnt = 0;
        let mut color_idx = 0;
        let mut balls_limits = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);
        let game = &line[idx+1..line.chars().count()];
        for (idx, val) in game.chars().enumerate() {
            //println!("Idx: {} val: {}", idx, val);
            if val == ' ' {
                continue;
            }
            if val == ';' || val == ',' || idx == game.chars().count()-1 {
                let ball_color = if idx == game.chars().count()-1 {
                    &game[color_idx..idx+1]
                } else {
                    &game[color_idx..idx]
                };
                if ball_cnt > *balls_limits.get(ball_color).unwrap() {
                    *balls_limits.get_mut(ball_color).unwrap() = ball_cnt;
                }
                ball_cnt = 0;
                color_idx = 0;
            } else if val >= '0' && val <= '9' {
                ball_cnt = ball_cnt * 10 + (val as i32 - '0' as i32);
            } else if color_idx == 0 {
                color_idx = idx;
            }
        }
        result += *balls_limits.get("red").unwrap() *  *balls_limits.get("green").unwrap() *  *balls_limits.get("blue").unwrap();
    }

    println!("Final result: {}", result);
}

fn d3p1_check(input_lines: &Vec<String>, row: usize, col_start: usize, col_stop: usize) -> bool {
    let mut result = false;
    let check_nbrs = |input_lines: &Vec<String>, row: usize, col_start: usize, col_stop: usize| -> bool {
        for idx in (col_start as i32 - 1)..(col_stop as i32 + 1) {
            if idx < 0 {
                continue;
            }
            if idx as usize >= input_lines[0].len() {
                continue;
            }
            let c = input_lines[row].chars().nth(idx as usize).unwrap();
            if c != '.' && (c <= '0' || c >= '9') {
                return true;
            }
        }
        return false;
    };
    if row > 0 {
        result |= check_nbrs(input_lines, row-1, col_start, col_stop);
    }

    if col_start > 0 {
        let c = input_lines[row].chars().nth(col_start - 1).unwrap();
        result |= c != '.' && (c <= '0' || c >= '9');
    }

    if col_stop < input_lines[0].chars().count() {
        let c = input_lines[row].chars().nth(col_stop).unwrap();
        result |= c != '.' && (c <= '0' || c >= '9');
    }

    if row+1 < input_lines.len() {
        result |= check_nbrs(input_lines, row+1, col_start, col_stop);
    }
    return result;
}

fn day_3_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 3 part 1");
    let mut result = 0;
    for (idx,line) in input_lines.into_iter().enumerate() {
        //println!("Parsing line {}: {}", idx, line);
        let mut val = 0;
        let mut start_idx = 0;
        for (jdx, c) in line.chars().enumerate() {
            if c >= '0' && c <= '9' {
                if val == 0 {
                    start_idx = jdx;
                }
                val = val * 10 + (c as i32 - '0' as i32);
            }

            let is_last_char = jdx == line.chars().count()-1;

            if c < '0' || c > '9' || is_last_char {
                let offset = if is_last_char {
                    1
                } else {
                    0
                };
                // process new number additional chack for numbers on the right edge
                if d3p1_check(input_lines, idx, start_idx, jdx + offset) {
                    result += val;
                }
                //else {
                //    println!("Invalid value: {}", val);
                //}
                val = 0;
            }
        }
    }
    println!("Final result: {}", result);
}

fn d3p2_check(input_lines: &Vec<String>, row: usize, col_start: usize, col_stop: usize) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = vec![];
    let check_nbrs = |input_lines: &Vec<String>, row: usize, col_start: usize, col_stop: usize, result: &mut Vec<(usize, usize)>| {
        for idx in (col_start as i32 - 1)..(col_stop as i32 + 1) {
            if idx < 0 {
                continue;
            }
            if idx as usize >= input_lines[0].len() {
                continue;
            }
            let c = input_lines[row].chars().nth(idx as usize).unwrap();
            if c == '*' {
                result.push((row, idx as usize))
            }
        }
    };
    if row > 0 {
        check_nbrs(input_lines, row-1, col_start, col_stop, &mut result);
    }

    if col_start > 0 {
        let c = input_lines[row].chars().nth(col_start - 1).unwrap();
        if c == '*' {
            result.push((row, col_start - 1));
        }
    }

    if col_stop < input_lines[0].chars().count() {
        let c = input_lines[row].chars().nth(col_stop).unwrap();
        if c == '*' {
            result.push((row, col_stop));
        }
    }

    if row+1 < input_lines.len() {
        check_nbrs(input_lines, row+1, col_start, col_stop, &mut result);
    }
    return result;
}

fn d3p2_update_lut(lut: &mut HashMap<(usize, usize), Vec<i32>>, positions: &Vec<(usize, usize)>, val: i32) {
    for position in positions.into_iter() {
        lut.entry(*position).and_modify(|nbrs| (*nbrs).push(val)).or_insert(vec![val]);
    }
}

fn day_3_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 3 part 2");
    let mut result = 0;
    let mut gears_lut: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    for (idx,line) in input_lines.into_iter().enumerate() {
        //println!("Parsing line {}: {}", idx, line);
        let mut val = 0;
        let mut start_idx = 0;
        for (jdx, c) in line.chars().enumerate() {
            if c >= '0' && c <= '9' {
                if val == 0 {
                    start_idx = jdx;
                }
                val = val * 10 + (c as i32 - '0' as i32);
            }

            let is_last_char = jdx == line.chars().count()-1;

            if (c < '0' || c > '9' || is_last_char) && val != 0 {
                // process new number additional check for numbers on the right edge
                let offset = if is_last_char {
                    1
                } else {
                    0
                };
                d3p2_update_lut(&mut gears_lut, &d3p2_check(input_lines, idx, start_idx, jdx + offset), val);
                val = 0;
            }
        }
    }
    for gear in gears_lut.into_iter() {
        let mut tmp = 1;
        //print!("Gear: {},{} ", gear.0.0, gear.0.1);
        if gear.1.len() != 2 {
            continue;
        }
        for v in gear.1 {
            //print!("{},", v);
            tmp = tmp * v;
        }
        result += tmp;
    }
    println!("Final result: {}", result);
}

fn day_4_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 4 part 1");
    let mut result = 0;
    for (idx, line) in input_lines.into_iter().enumerate() {
        println!("Parsing line {}: {}", idx, line);
        let mut jdx = 5;
        let mut card_num = 0;
        let mut winning: Vec<i32> = vec![];
        let mut chosen: Vec<i32> = vec![];
        while jdx < line.chars().count() {
            // Get card number
            let c = line.chars().nth(jdx).unwrap();
            jdx += 1;
            if c == ':' {
                break;
            }
            if c >= '0' && c <= '9' {
                card_num = card_num * 10 + (c as i32 - '0' as i32);
            }
        }

        let mut val = 0;
        while jdx < line.chars().count() {
            // Get winning numbers
            let c = line.chars().nth(jdx).unwrap();
            if c == '|' {
                break;
            }

            if c == ' ' && val > 0 {
                winning.push(val);
                val = 0;
            } else if c >= '0' && c <= '9' {
                val = val * 10 + (c as i32 - '0' as i32);
            }
            jdx += 1;
        }

        val = 0;
        while jdx < line.chars().count() {
            // Get chosen numbers
            let c = line.chars().nth(jdx).unwrap();

            if c == ' ' && val > 0 {
                chosen.push(val);
                val = 0;
            } else if c >= '0' && c <= '9' {
                val = val * 10 + (c as i32 - '0' as i32);
            }
            jdx += 1;
        }

        if val > 0 {
            chosen.push(val);
        }

        val = 0;
        let winning_set: HashSet<i32> = HashSet::from_iter(winning.into_iter());
        for v in chosen {
            if winning_set.contains(&v) {
                val += 1;
            }
        }
        if val > 0 {
            result += 1 << (val-1);
        }
    }
    println!("Final result: {}", result);
}

fn day_4_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 4 part 2");
    let mut result = 0;
    let mut cards_cnt: HashMap<i32, i32> = HashMap::from([]);
    for (idx, line) in input_lines.into_iter().enumerate() {
        println!("Parsing line {}: {}", idx, line);
        let mut jdx = 5;
        let mut card_num = 0;
        let mut winning: Vec<i32> = vec![];
        let mut chosen: Vec<i32> = vec![];
        while jdx < line.chars().count() {
            // Get card number
            let c = line.chars().nth(jdx).unwrap();
            jdx += 1;
            if c == ':' {
                break;
            }
            if c >= '0' && c <= '9' {
                card_num = card_num * 10 + (c as i32 - '0' as i32);
            }
        }

        let mut val = 0;
        while jdx < line.chars().count() {
            // Get winning numbers
            let c = line.chars().nth(jdx).unwrap();
            if c == '|' {
                break;
            }

            if c == ' ' && val > 0 {
                winning.push(val);
                val = 0;
            } else if c >= '0' && c <= '9' {
                val = val * 10 + (c as i32 - '0' as i32);
            }
            jdx += 1;
        }

        val = 0;
        while jdx < line.chars().count() {
            // Get chosen numbers
            let c = line.chars().nth(jdx).unwrap();

            if c == ' ' && val > 0 {
                chosen.push(val);
                val = 0;
            } else if c >= '0' && c <= '9' {
                val = val * 10 + (c as i32 - '0' as i32);
            }
            jdx += 1;
        }

        if val > 0 {
            chosen.push(val);
        }

        val = 0;
        let winning_set: HashSet<i32> = HashSet::from_iter(winning.into_iter());
        for v in chosen {
            if winning_set.contains(&v) {
                val += 1;
            }
        }

        cards_cnt.entry(card_num).or_insert(1);
        for idx in 1..val + 1 {
            let multiplier = cards_cnt[&card_num];
            if card_num + idx <= input_lines.len() as i32 {
                println!("Parsing card: {} multiplier: {} idx: {}", card_num, multiplier, idx);
                *cards_cnt.entry(card_num+idx).or_insert(1) += multiplier;
            } else {
                println!("Index out of bounds: {}", card_num + idx);
            }
        }
    }
    for card_cnt in cards_cnt {
        println!("Card {}:{}", card_cnt.0, card_cnt.1);
        result += card_cnt.1;
    }
    println!("Final result: {}", result);
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum D5P1State {
    Idle,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug, Clone)]
struct D5P1Range {
    src: u64,
    dst: u64,
    len: u64,
}

fn d5p1_parse_map(line: &String, ranges: &mut Vec<D5P1Range>) {
    let vals: Vec<&str> = line.split(" ").collect();
    assert_eq!(vals.len(), 3);
    let dst = vals[0].parse::<u64>().unwrap();
    let src = vals[1].parse::<u64>().unwrap();
    let len = vals[2].parse::<u64>().unwrap();
    ranges.push(D5P1Range{src: src, dst: dst, len: len});

}

fn d5p1_get_dst(src: u64, ranges: &Vec<D5P1Range>) -> u64 {
    for range in ranges {
        if src >= range.src && src < (range.src + range.len) {
            return range.dst + (src - range.src);
        }
    }
    return src;
}

fn day_5_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 5 part 2");
    let mut result: u64 = u64::MAX;
    let mut state: D5P1State = D5P1State::Idle;
    let mut states_lut: HashMap<D5P1State, Vec<D5P1Range>> = HashMap::new();
    let mut tmp_ranges: Vec<D5P1Range> = vec![];
    let mut seeds: Vec<&str> = vec![];
    for (idx, line) in input_lines.into_iter().enumerate() {
        println!("Parsing line {}: {}", idx, line);
        if line.is_empty() {
            println!("Skipping empty line.");
            if !matches!(state, D5P1State::Idle) {
                println!("Current state: {:?}", state);
                states_lut.insert(state, tmp_ranges.clone());
                tmp_ranges.clear();
            }
            state = D5P1State::Idle;
            continue;
        }

        if line.starts_with("seeds: ") {
            seeds = line.split_at(7).1.split(' ').collect();
            println!("Seeds: {:?}", seeds);
        }
        else if line.starts_with("seed-to-soil map:") {
            state = D5P1State::SeedToSoil;
        }
        else if line.starts_with("soil-to-fertilizer map:") {
            state = D5P1State::SoilToFertilizer;
        }
        else if line.starts_with("fertilizer-to-water map:") {
            state = D5P1State::FertilizerToWater;
        }
        else if line.starts_with("water-to-light map:") {
            state = D5P1State::WaterToLight;
        }
        else if line.starts_with("light-to-temperature map:") {
            state = D5P1State::LightToTemperature;
        }
        else if line.starts_with("temperature-to-humidity map:") {
            state = D5P1State::TemperatureToHumidity;
        }
        else if line.starts_with("humidity-to-location map:") {
            state = D5P1State::HumidityToLocation;
        } else {
            println!("Parsing map entry: {}", line);
            d5p1_parse_map(&line, &mut tmp_ranges);
        }
    }

    for seed in seeds {
        let mut tmp_src: u64 = seed.parse().unwrap();
        println!("Processing seed: {}", tmp_src);
        tmp_src = d5p1_get_dst(tmp_src, &states_lut.get(&D5P1State::SeedToSoil).unwrap());
        tmp_src = d5p1_get_dst(tmp_src, &states_lut.get(&D5P1State::SoilToFertilizer).unwrap());
        tmp_src = d5p1_get_dst(tmp_src, &states_lut.get(&D5P1State::FertilizerToWater).unwrap());
        tmp_src = d5p1_get_dst(tmp_src, &states_lut.get(&D5P1State::WaterToLight).unwrap());
        tmp_src = d5p1_get_dst(tmp_src, &states_lut.get(&D5P1State::LightToTemperature).unwrap());
        tmp_src = d5p1_get_dst(tmp_src, &states_lut.get(&D5P1State::TemperatureToHumidity).unwrap());
        tmp_src = d5p1_get_dst(tmp_src, &states_lut.get(&D5P1State::HumidityToLocation).unwrap());
        if result > tmp_src {
            result = tmp_src;
        }
    }
    println!("Final result: {}", result);
}

#[derive(Debug, Clone)]
struct D5P2Range {
    src: u64,
    len: u64,
}

fn d5p2_get_dst<'a>(seed: &D5P2Range, ranges: &'a Vec<D5P1Range>) -> Option<&'a D5P1Range> {
    for range in ranges {
        if seed.src >= range.src && seed.src < (range.src + range.len) {
            // Start of range fits
            return Some(&range);
        } else if (seed.src + seed.len) >= range.src && (seed.src + seed.len) < (range.src + range.len) {
            // End of range fits
            return Some(&range);
        }
    }
    // Seed did not fit any range so return 1:1 mapping
    None
}

fn d5p2_process(seeds: &mut Vec<D5P2Range>, lut: &Vec<D5P1Range>) -> Vec<D5P2Range> {
    let mut result: Vec<D5P2Range> = vec![];
    while !seeds.is_empty() {
        let seed = seeds.pop().unwrap();
        println!("Parsing seeds at: {} range: {}", seed.src, seed.len);
        let tmp_seed = d5p2_get_dst(&seed, lut);
        if !tmp_seed.is_some() {
            // Empty result means 1:1 mapping
            result.push(seed);
        } else {
            let tmp_range = tmp_seed.unwrap();
            if seed.src >= tmp_range.src && seed.src + seed.len < tmp_range.src + tmp_range.len {
                // All current range fits in new mapping
                println!("Range fits");
                result.push(D5P2Range { src: tmp_range.dst + seed.src - tmp_range.src, len: seed.len });
            } else if seed.src >= tmp_range.src && seed.src + seed.len >= tmp_range.src + tmp_range.len {
                // Begining of current range fits in new mapping
                println!("Left side of the range fits");
                let offset = seed.src - tmp_range.src;
                let fit_len = tmp_range.len - offset;
                result.push(D5P2Range { src: tmp_range.dst + offset, len: fit_len });
                seeds.push(D5P2Range { src: tmp_range.src + tmp_range.len, len: seed.len - fit_len });
            } else {
                // End of current range fits in new mapping
                println!("Right side of the range fits[src {} {}| range {} {}]", seed.src, seed.len, tmp_range.src, tmp_range.len);
                // Add one because first element from the tmp_range is also parsed already
                let fit_len = seed.len - (tmp_range.src - seed.src) + 1;
                result.push(D5P2Range { src: tmp_range.dst, len: fit_len });
                seeds.push(D5P2Range { src: seed.src, len: seed.len - fit_len });
            }
        }
        //result.push(tmp_seed);
    }
    return result;
}

fn day_5_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 5 part 2");
    let mut result: u64 = u64::MAX;
    let mut state: D5P1State = D5P1State::Idle;
    let mut states_lut: HashMap<D5P1State, Vec<D5P1Range>> = HashMap::new();
    let mut tmp_ranges: Vec<D5P1Range> = vec![];
    let mut seeds: Vec<&str> = vec![];
    for (idx, line) in input_lines.into_iter().enumerate() {
        println!("Parsing line {}: {}", idx, line);
        if line.is_empty() {
            //println!("Skipping empty line.");
            if !matches!(state, D5P1State::Idle) {
                //println!("Current state: {:?}", state);
                states_lut.insert(state, tmp_ranges.clone());
                tmp_ranges.clear();
            }
            state = D5P1State::Idle;
            continue;
        }

        if line.starts_with("seeds: ") {
            seeds = line.split_at(7).1.split(' ').collect();
            //println!("Seeds: {:?}", seeds);
        }
        else if line.starts_with("seed-to-soil map:") {
            state = D5P1State::SeedToSoil;
        }
        else if line.starts_with("soil-to-fertilizer map:") {
            state = D5P1State::SoilToFertilizer;
        }
        else if line.starts_with("fertilizer-to-water map:") {
            state = D5P1State::FertilizerToWater;
        }
        else if line.starts_with("water-to-light map:") {
            state = D5P1State::WaterToLight;
        }
        else if line.starts_with("light-to-temperature map:") {
            state = D5P1State::LightToTemperature;
        }
        else if line.starts_with("temperature-to-humidity map:") {
            state = D5P1State::TemperatureToHumidity;
        }
        else if line.starts_with("humidity-to-location map:") {
            state = D5P1State::HumidityToLocation;
        } else {
            //println!("Parsing map entry: {}", line);
            d5p1_parse_map(&line, &mut tmp_ranges);
        }
    }

    assert_eq!(seeds.len()%2, 0);
    let mut tmp_src: Vec<D5P2Range> = vec![];
    for seed in seeds.chunks(2) {
        println!("Processing seed at: {} range: {}", seed[0], seed[1]);
        tmp_src.push(D5P2Range { src: seed[0].parse().unwrap(), len: seed[1].parse().unwrap() })
    }
    println!("Parsing SeedToSoil");
    tmp_src = d5p2_process(&mut tmp_src, &states_lut.get(&D5P1State::SeedToSoil).unwrap());
    println!("Parsing SoilToFertilizer");
    tmp_src = d5p2_process(&mut tmp_src, &states_lut.get(&D5P1State::SoilToFertilizer).unwrap());
    println!("Parsing FertilizerToWater");
    tmp_src = d5p2_process(&mut tmp_src, &states_lut.get(&D5P1State::FertilizerToWater).unwrap());
    println!("Parsing WaterToLight");
    tmp_src = d5p2_process(&mut tmp_src, &states_lut.get(&D5P1State::WaterToLight).unwrap());
    println!("Parsing LightToTemperature");
    tmp_src = d5p2_process(&mut tmp_src, &states_lut.get(&D5P1State::LightToTemperature).unwrap());
    println!("Parsing TemperatureToHumidity");
    tmp_src = d5p2_process(&mut tmp_src, &states_lut.get(&D5P1State::TemperatureToHumidity).unwrap());
    println!("Parsing HumidityToLocation");
    tmp_src = d5p2_process(&mut tmp_src, &states_lut.get(&D5P1State::HumidityToLocation).unwrap());
    println!("Parsing finished");
    for entry in tmp_src {
        if entry.src < result {
            result = entry.src;
        }
    }
    println!("Final result: {}", result);
}

fn day_6_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 6 part 1");
    let mut result = 1;

    let times: Vec<&str> = input_lines[0].split(' ').filter(|x| x.len() > 0).collect();
    let distances: Vec<&str> = input_lines[1].split(' ').filter(|x| x.len() > 0).collect();
    assert_eq!(times[0], "Time:");
    assert_eq!(distances[0], "Distance:");
    assert_eq!(times.len(), distances.len());
    for idx in 1..times.len() {
        let time: u64 = times[idx].parse().unwrap();
        let distance: u64 = distances[idx].parse().unwrap();
        println!("Parsing time: {} and distance: {}", time, distance);
        let valid: Vec<u64>= (1..time).filter(|x| x * (time - x) > distance).collect();
        println!("Possible times: {:?}", valid);
        result = result * valid.len();
    }
    println!("Final result: {}", result);
}

fn day_6_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 6 part 2");
    let mut result = 0;

    let times: Vec<&str> = input_lines[0].split(' ').filter(|x| x.len() > 0).skip(1).collect();
    let distances: Vec<&str> = input_lines[1].split(' ').filter(|x| x.len() > 0).skip(1).collect();
    let time: u64 = times.join("").parse().unwrap();
    let distance: u64 = distances.join("").parse().unwrap();
    println!("Count for time: {} and distance: {}", time, distance);
    for i in 1..time {
        if i * (time - i) > distance {
            result = time - i - i + 1;
            break;
        }
    }
    println!("Final result: {}", result);
}

#[derive(Debug, PartialEq, Hash, Eq)]
enum D7P1Type {
    FiveOfKind = 6,
    FourOfKind = 5,
    FullHouse = 4,
    ThreeOfKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug)]
struct D7P1Hand {
    hand: String,
    name: D7P1Type,
    bid: u64,
}

fn d7p1_hand_to_type(input: &str) -> D7P1Type {
    let stats = input.to_string().chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    println!("Preparsed {:?}", stats);
    if stats.len() == 5 {
        // 1 + 1 + 1 + 1 + 1
        return D7P1Type::HighCard;
    } else if stats.len() == 4 {
        // 2 + 1 + 1 + 1
        return D7P1Type::OnePair;
    } else if stats.len() == 3 {
        // 3 + 1 + 1, 2 + 2 + 1
        for (_, val) in stats.iter() {
            if *val == 2 {
                return D7P1Type::TwoPair;
            }
        }
        return D7P1Type::ThreeOfKind;
    } else if stats.len() == 2 {
        // 4 + 1, 3 + 2
        let tmp = stats.get(stats.keys().next().unwrap()).unwrap();
        if tmp == &4 || tmp == &1 {
            return D7P1Type::FourOfKind;
        }
        return D7P1Type::FullHouse;
    } else if stats.len() == 1 {
        return D7P1Type::FiveOfKind;
    }
    D7P1Type::HighCard
}

fn d7p1_compare(a: &D7P1Hand, b: &D7P1Hand) -> std::cmp::Ordering {
    let lut: HashMap<char, u8> = HashMap::from([
        ('2', 0),
        ('3', 1),
        ('4', 2),
        ('5', 3),
        ('6', 4),
        ('7', 5),
        ('8', 6),
        ('9', 7),
        ('T', 8),
        ('J', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]);

    let lut_h: HashMap<D7P1Type, u8> = HashMap::from([
        (D7P1Type::HighCard, 0),
        (D7P1Type::OnePair, 1),
        (D7P1Type::TwoPair, 2),
        (D7P1Type::ThreeOfKind, 3),
        (D7P1Type::FullHouse, 4),
        (D7P1Type::FourOfKind, 5),
        (D7P1Type::FiveOfKind, 6),
    ]);
    if a.name == b.name {
        // hands are equal check cards starting from first
        for i in 0..a.hand.len() {
            let tmp_a = a.hand.chars().nth(i).unwrap();
            let tmp_b = b.hand.chars().nth(i).unwrap();
            if tmp_a == tmp_b {
                continue;
            }
            if lut.get(&tmp_a).unwrap() > lut.get(&tmp_b).unwrap() {
                return std::cmp::Ordering::Greater;
            }
            return std::cmp::Ordering::Less;
        }
    } else {
        if lut_h.get(&a.name).unwrap() > lut_h.get(&b.name).unwrap() {
            return std::cmp::Ordering::Greater;
        } else {
            return std::cmp::Ordering::Less;
        }
    }
    return std::cmp::Ordering::Greater;
}

fn d7p1_parse_input(input: &String) -> D7P1Hand {
    let val: Vec<_> = input.split(' ').collect();
    return D7P1Hand { hand: val[0].to_string(), name: d7p1_hand_to_type(val[0]), bid: val[1].parse().unwrap() }
}

fn day_7_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 7 part 1");
    let mut result: u64 = 0;
    let mut hands: Vec<D7P1Hand> = vec![];
    for (idx, line) in input_lines.into_iter().enumerate() {
        println!("Parsing line {}: {}", idx, line);
        hands.push(d7p1_parse_input(line))
    }
    hands.sort_by(d7p1_compare);
    for (idx, hand) in hands.iter().enumerate() {
        println!("Parsed hand: {} {:?}", idx+1, hand);
        result += (idx as u64 + 1) * hand.bid;
    }
    println!("Final result: {}", result);
}

fn d7p2_hand_to_type(input: &str) -> D7P1Type {
    let stats = input.to_string().chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    println!("Preparsed {:?}", stats);
    let js = stats.get(&'J');
    if stats.len() == 1 {
        return D7P1Type::FiveOfKind;
    } else if stats.len() == 5 {
        // 1 + 1 + 1 + 1 + 1
        if js.is_some() {
            return D7P1Type::OnePair;
        }
        return D7P1Type::HighCard;
    } else if stats.len() == 4 {
        // 2 + 1 + 1 + 1
        if js.is_some() {
            // Either 2J + 1X, else 1J + 2X
            return D7P1Type::ThreeOfKind;
        }
        return D7P1Type::OnePair;
    } else if stats.len() == 3 {
        // 3 + 1 + 1, 2 + 2 + 1
        if js.is_some() {
            let tmp = js.unwrap();
            for (key, val) in stats.iter() {
                if key == &'J' {
                    continue;
                }
                if *val == 2 {
                    if tmp == &2 {
                        return D7P1Type::FourOfKind;
                    } else {
                        return D7P1Type::FullHouse;
                    }
                }
            }
            return D7P1Type::FourOfKind;
        }
        for (_, val) in stats.iter() {
            if *val == 2 {
                return D7P1Type::TwoPair;
            }
        }
        return D7P1Type::ThreeOfKind;
    } else if stats.len() == 2 {
        // 4 + 1, 3 + 2
        let tmp = stats.get(stats.keys().next().unwrap()).unwrap();
        if js.is_some() {
            return D7P1Type::FiveOfKind;
        }
        if tmp == &4 || tmp == &1 {
            return D7P1Type::FourOfKind;
        }
        return D7P1Type::FullHouse;
    }
    D7P1Type::HighCard
}

fn d7p2_compare(a: &D7P1Hand, b: &D7P1Hand) -> std::cmp::Ordering {
    let lut: HashMap<char, u8> = HashMap::from([
        ('J', 0),
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]);

    let lut_h: HashMap<D7P1Type, u8> = HashMap::from([
        (D7P1Type::HighCard, 0),
        (D7P1Type::OnePair, 1),
        (D7P1Type::TwoPair, 2),
        (D7P1Type::ThreeOfKind, 3),
        (D7P1Type::FullHouse, 4),
        (D7P1Type::FourOfKind, 5),
        (D7P1Type::FiveOfKind, 6),
    ]);
    if a.name == b.name {
        // hands are equal check cards starting from first
        for i in 0..a.hand.len() {
            let tmp_a = a.hand.chars().nth(i).unwrap();
            let tmp_b = b.hand.chars().nth(i).unwrap();
            if tmp_a == tmp_b {
                continue;
            }
            if lut.get(&tmp_a).unwrap() > lut.get(&tmp_b).unwrap() {
                return std::cmp::Ordering::Greater;
            }
            return std::cmp::Ordering::Less;
        }
    } else {
        if lut_h.get(&a.name).unwrap() > lut_h.get(&b.name).unwrap() {
            return std::cmp::Ordering::Greater;
        } else {
            return std::cmp::Ordering::Less;
        }
    }
    return std::cmp::Ordering::Greater;
}

fn d7p2_parse_input(input: &String) -> D7P1Hand {
    let val: Vec<_> = input.split(' ').collect();
    return D7P1Hand { hand: val[0].to_string(), name: d7p2_hand_to_type(val[0]), bid: val[1].parse().unwrap() }
}

fn day_7_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 7 part 2");
    let mut result: u64 = 0;
    let mut hands: Vec<D7P1Hand> = vec![];
    for (idx, line) in input_lines.into_iter().enumerate() {
        println!("Parsing line {}: {}", idx, line);
        hands.push(d7p2_parse_input(line))
    }
    hands.sort_by(d7p2_compare);
    println!("Sorted: {:?}", hands);
    for (idx, hand) in hands.iter().enumerate() {
        if hand.hand.find('J').is_some() {
            println!("Parsed hand: {} {:?}", idx+1, hand);
        }
        result += (idx as u64 + 1) * hand.bid;
    }
    println!("Final result: {}", result);
}

fn day_8_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 8 part 1");
    let mut result = 0;

    let directions = input_lines.get(0).unwrap();
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    println!("Directions: {}", directions);
    for (_,line) in input_lines.into_iter().skip(2).enumerate() {
        let tmp: Vec<&str> = line.split(' ').collect();
        assert_eq!(tmp.len(), 4);
        println!("{:?}", tmp);
        nodes.insert(tmp[0], (&tmp[2][1..tmp[2].len()-1], &tmp[3][..tmp[3].len()-1]));
    }
    println!("{:?}", nodes);
    let mut current = "AAA";
    while current != "ZZZ" {
        if directions.chars().nth(result%directions.len()).unwrap() == 'R' {
            current = nodes.get(current).unwrap().1;
        } else {
            current = nodes.get(current).unwrap().0;
        }
        result += 1;
    }
    println!("Final result: {}", result);
}

fn day_8_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 8 part 2");
    let mut result: u64;
    let mut results: Vec<u64> = vec![];

    let directions = input_lines.get(0).unwrap();
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut currents: Vec<&str> = vec![];

    println!("Directions: {}", directions);
    for (_,line) in input_lines.into_iter().skip(2).enumerate() {
        let tmp: Vec<&str> = line.split(' ').collect();
        assert_eq!(tmp.len(), 4);
        println!("{:?}", tmp);
        nodes.insert(tmp[0], (&tmp[2][1..tmp[2].len()-1], &tmp[3][..tmp[3].len()-1]));
        if tmp[0].ends_with("A") {
            currents.push(&tmp[0]);
        }
    }
    println!("Nodes: {:?}", nodes);
    println!("Start: {:?}", currents);
    for mut current in currents {
        println!("Parsing: {}", current);
        result = 0;
        while !current.ends_with('Z') {
            if directions.chars().nth(result as usize%directions.len()).unwrap() == 'R' {
                current = nodes.get(current).unwrap().1;
            } else {
                current = nodes.get(current).unwrap().0;
            }
            result += 1;
        }
        results.push(result);
    }
    println!("Final result: {:?}", results);
    result = 0;
    for tmp in results {
        println!("Tmp result: {} {}", tmp, result);
        if result == 0 {
            result = tmp;
        } else {
            result = aoc_lcm(tmp, result);
        }
    }
    println!("Final result: {}", result);
}

fn d9p1_get_next_step(vals: &Vec<i64>) -> i64 {
    //println!("Values to parse: {:?}", vals);
    let mut diffs: Vec<i64> = vec![];
    let mut all_same = true;
    for idx in 1..vals.len() {
        let diff = vals[idx] - vals[idx-1];
        diffs.push(diff);
        if diffs.first().unwrap() != &diff {
            all_same = false;
        }
    }
    if all_same {
        return *vals.last().unwrap() + *diffs.first().unwrap();
    }
    return *vals.last().unwrap() + d9p1_get_next_step(&diffs);
}

fn day_9_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 9 part 1");
    let mut result = 0;

    for (idx ,line) in input_lines.into_iter().enumerate() {
        let series: Vec<i64> = line.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
        let next = d9p1_get_next_step(&series);
        result += next;
        println!("Idx: {} Next: {} Intermediate result: {}", idx, next, result);
    }
    println!("Final result: {}", result);
}

fn d9p2_get_prev_step(vals: &Vec<i64>) -> i64 {
    //println!("Values to parse: {:?}", vals);
    let mut diffs: Vec<i64> = vec![];
    let mut all_same = true;
    for idx in 1..vals.len() {
        let diff = vals[idx] - vals[idx-1];
        diffs.push(diff);
        if diffs.first().unwrap() != &diff {
            all_same = false;
        }
    }
    if all_same {
        return *vals.first().unwrap() - *diffs.first().unwrap();
    }
    return *vals.first().unwrap() - d9p2_get_prev_step(&diffs);
}

fn day_9_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 9 part 2");
    let mut result = 0;

    for (idx ,line) in input_lines.into_iter().enumerate() {
        println!("{}: {}", idx, line);
        let series: Vec<i64> = line.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
        let next = d9p2_get_prev_step(&series);
        result += next;
        println!("Idx: {} Next: {} Intermediate result: {}", idx, next, result);
    }
    println!("Final result: {}", result);
}

fn d10p1_is_valid_move(map: &Vec<String>, pos: (i16, i16)) -> bool {
    if pos.0 < 0 || pos.1 < 0 {
        return false;
    }
    if pos.0 >= map.first().unwrap().len() as i16 || pos.1 >= map.len() as i16 {
        return false;
    }
    true
}

fn d10p1_get_next(map: &Vec<String>, pos: &(i16, i16), visited: &HashSet<(i16, i16)>, nexts: &mut Vec<(i16, i16)>) {
    let steps: HashMap<char, Vec<(i16, i16)>> = HashMap::from([
        ('|', vec![(0, -1), (0, 1)]),
        ('-', vec![(-1, 0), (1, 0)]),
        ('L', vec![(0, -1), (1, 0)]),
        ('J', vec![(0, -1), (-1, 0)]),
        ('7', vec![(0, 1), (-1, 0)]),
        ('F', vec![(0, 1), (1, 0)]),
        ('.', vec![]),
        ('S', vec![(-1, 0), (1, 0), (0, -1), (0, 1)]),
    ]);

    let is_valid_nbr = |pos: &(i16, i16), valid_pos: &Vec<(i16, i16)>| -> bool {
        // check if pipe on next position is valid neighbor
        // println!("Validating nbr at: {:?} against: {:?}", pos, valid_pos);
        for tmp in valid_pos {
            if tmp.0 + pos.0 == 0 && tmp.1 + pos.1 == 0 {
                return true;
            }
        }
        return false;
    };
    //println!("Getting nexts for {:?}", pos);
    let current_pipe = map[pos.1 as usize].chars().nth(pos.0 as usize).unwrap();
    assert!(steps.contains_key(&current_pipe));
    let next_steps = steps.get(&current_pipe).unwrap();
    for tmp in next_steps {
        let next_pos = (pos.0 + tmp.0, pos.1 + tmp.1);
        if !d10p1_is_valid_move(map, next_pos) {
            continue;
        }
        let next_pipe = map[next_pos.1 as usize].chars().nth(next_pos.0 as usize).unwrap();
        //println!("Checking: {} at {:?}/{:?}", next_pipe, next_pos, tmp);
        if is_valid_nbr(tmp, steps.get(&next_pipe).unwrap()) && !visited.contains(&next_pos) {
            nexts.push(next_pos);
        }
    }
}

fn day_10_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 10 part 1");
    let result;
    let mut start = (-1, -1);

    for (idx ,line) in input_lines.into_iter().enumerate() {
        println!("{}: {}", idx, line);
        if line.contains('S') {
            start = (line.find('S').unwrap() as i16, idx as i16);
        }
    }

    let mut tmp_map = input_lines.clone();
    for line in tmp_map.iter_mut() {
        *line = std::iter::repeat('.').take(line.len()).collect::<String>()
    }

    println!("Starting at: {:?}", start);
    let mut nexts:Vec<(i16, i16)> = vec![start];
    let mut visited: HashSet<(i16, i16)> = HashSet::new();
    while !nexts.is_empty() {
        let next = nexts.pop().unwrap();
        tmp_map[next.1 as usize].replace_range(next.0 as usize..next.0 as usize +1, "#");
        visited.insert(next);
        d10p1_get_next(input_lines, &next, &visited, &mut nexts);
    }
    for line in tmp_map {
        println!("{:?}", line);
    }
    result = visited.len();
    println!("Visited nodes: {}", result);
    println!("Final result: {}", result/2);
}

fn d10p2_get_shape(map: &Vec<String>, visited: &HashSet<(i16, i16)>, start: &(i16, i16)) -> char {
    let offsets: Vec<(i16, i16)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut nbrs: HashSet<(i16, i16)> = HashSet::new();
    for offset in offsets {
        let tmp = (start.0 + offset.0,start.1 + offset.1);
        if visited.contains(&tmp) {
            let nbr_shape = map[tmp.1 as usize].chars().nth(tmp.0 as usize).unwrap();
            if nbr_shape == '|' && (offset == (-1, 0) || offset == (1, 0)) {
                continue;
            } else if nbr_shape == '-' && (offset == (0, -1) || offset == (0, 1)) {
                continue;
            } else if nbr_shape == 'F' && (offset == (1, 0) || offset == (0, 1)) {
                continue;
            } else if nbr_shape == 'L' && (offset == (1, 0) || offset == (0, -1)) {
                continue;
            } else if nbr_shape == 'J' && (offset == (-1, 0) || offset == (0, -1)) {
                continue;
            } else if nbr_shape == '7' && (offset == (-1, 0) || offset == (0, 1)) {
                continue;
            }
            nbrs.insert(offset);
        }
    }
    println!("Nbrs: {:?}", nbrs);
    assert_eq!(nbrs.len(), 2);
    if nbrs.contains(&(-1, 0)) && nbrs.contains(&(1, 0)) {
        return '-';
    } else if nbrs.contains(&(0, -1)) && nbrs.contains(&(0, 1)) {
        return '|';
    } else if nbrs.contains(&(0, 1)) && nbrs.contains(&(1, 0)) {
        return 'F';
    } else if nbrs.contains(&(0, -1)) && nbrs.contains(&(1, 0)) {
        return 'L';
    } else if nbrs.contains(&(0, -1)) && nbrs.contains(&(-1, 0)) {
        return 'J';
    } else if nbrs.contains(&(0, 1)) && nbrs.contains(&(-1, 0)) {
        return '7';
    }
    return 'X';
}

fn day_10_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 10 part 2");
    let mut result = 0;
    let mut start = (-1, -1);

    for (idx ,line) in input_lines.into_iter().enumerate() {
        println!("{}: {}", idx, line);
        if line.contains('S') {
            start = (line.find('S').unwrap() as i16, idx as i16);
        }
    }

    let mut tmp_map = input_lines.clone();
    for line in tmp_map.iter_mut() {
        *line = std::iter::repeat('.').take(line.len()).collect::<String>()
    }

    println!("Starting at: {:?}", start);
    let mut nexts:Vec<(i16, i16)> = vec![start];
    let mut visited: HashSet<(i16, i16)> = HashSet::new();
    while !nexts.is_empty() {
        let next = nexts.pop().unwrap();
        tmp_map[next.1 as usize].replace_range(next.0 as usize..next.0 as usize +1, "#");
        visited.insert(next);
        d10p1_get_next(input_lines, &next, &visited, &mut nexts);
    }

    for row in 0..input_lines.len() {
        for col in 0..input_lines[row].len() {
            if visited.contains(&(col as i16, row as i16)) {
                continue;
            }
            let mut edge_counter = 0;
            let mut prev: char = ' ';
            for (idx, mut val) in input_lines[row].chars().into_iter().enumerate() {
                if idx >= col {
                    break;
                }
                if !visited.contains(&(idx as i16, row as i16)) {
                    // we scann all characters in the line, previous check does not apply here
                    continue;
                }

                if val == '-' {
                    continue;
                }

                if val == 'S' {
                    // we need to convert S to proper shape to proprly count number of crossed
                    // edges
                    val = d10p2_get_shape(&input_lines, &visited, &start);
                    println!("Replacing S to {}", prev);
                }

                if  val == '|' {
                    edge_counter += 1;
                } else {
                    if val == '7' && (prev == 'L' || prev == 'S') {
                        edge_counter += 1;
                    } else if val == 'J' && (prev == 'F' || prev == 'S') {
                        edge_counter += 1;
                    }
                }

                if val != '|' && val != '-' {
                    prev = val;
                }
            }

            //println!("Pos: {:?} edges: {}", (col, row), edge_counter);
            if edge_counter > 0 && edge_counter%2 == 1 {
                // we crossed the edge of pipe "circle" odd number of times which means we are
                // inside the "circle", read about ray casting to learn more about it
                result += 1;
            }
        }
    }

    for line in tmp_map {
        println!("{:?}", line);
    }
    println!("Final result: {}", result);
}

fn day_11_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 11 part 1");
    let mut result = 0;
    // We need to clone the input becasue we have to parse and modify it
    let mut input_parsed: Vec<String> = vec![];

    // Expand rows
    for (_, line) in input_lines.into_iter().enumerate() {
        input_parsed.push(line.to_string());
        if !line.contains("#") {
            input_parsed.push(line.to_string());
        }
    }

    // Find empty columns
    let mut empty_columns: Vec<usize> = vec![];
    for idx in 0..input_parsed[0].len() {
        let mut empty: bool = true;
        for line in input_lines {
            if line.chars().nth(idx).unwrap() != '.' {
                empty = false;
                break;
            }
        }

        if empty {
            println!("Idx: {} Offset: {}", idx, empty_columns.len());
            empty_columns.push(idx + empty_columns.len());
        }
    }

    // Expand columns
    println!("Empty columns: {:?}", empty_columns);
    for col in empty_columns {
        for idx in 0..input_parsed.len() {
            input_parsed[idx].insert(col, '.');
        }
    }

    let mut galaxies: Vec<(i32, i32)> = vec![];
    for (idx, line) in input_parsed.into_iter().enumerate() {
        for (jdx, c) in line.chars().into_iter().enumerate() {
            if c != '.' {
                galaxies.push((idx as i32, jdx as i32));
            }
        }
    }
    println!("Galaxies to check: {:?}", galaxies);
    for idx in 0..galaxies.len() {
        for jdx in idx+1..galaxies.len() {
            let a = galaxies[idx];
            let b = galaxies[jdx];
            result += (a.0-b.0).abs() + (a.1-b.1).abs();
        }
    }
    println!("Final result: {}", result);
}

fn day_11_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 11 part 2");
    let mut result: u64 = 0;
    let mut empty_rows: Vec<usize> = vec![];
    let mut empty_cols: Vec<usize> = vec![];

    // Expand rows
    for (idx, line) in input_lines.into_iter().enumerate() {
        if !line.contains("#") {
            empty_rows.push(idx);
        }
    }

    // Find empty columns
    for idx in 0..input_lines[0].len() {
        let mut empty: bool = true;
        for line in input_lines {
            if line.chars().nth(idx).unwrap() != '.' {
                empty = false;
                break;
            }
        }

        if empty {
            empty_cols.push(idx);
        }
    }

    println!("Empty rows: {:?}", empty_rows);
    println!("Empty columns: {:?}", empty_cols);

    let mut galaxies: Vec<(i32, i32)> = vec![];
    for (idx, line) in input_lines.into_iter().enumerate() {
        for (jdx, c) in line.chars().into_iter().enumerate() {
            if c != '.' {
                galaxies.push((idx as i32, jdx as i32));
            }
        }
    }
    println!("Galaxies to check: {:?}", galaxies);
    for idx in 0..galaxies.len() {
        let multiplier = 1000000;
        for jdx in idx+1..galaxies.len() {
            let a = galaxies[idx];
            let b = galaxies[jdx];
            result += (a.0-b.0).abs() as u64 + (a.1-b.1).abs() as u64;

            for row in empty_rows.iter() {
                let r = *row as i32;
                if r > a.0 && r < b.0 || r < a.0 && r > b.0 {
                    result += multiplier - 1;
                }
            }
            for col in empty_cols.iter() {
                let c = *col as i32;
                if c > a.1 && c < b.1 || c < a.1 && c > b.1 {
                    result += multiplier - 1;
                }
            }
        }
    }
    println!("Final result: {}", result);
}

fn d12_p1_valid_chain(springs: &str, len: i32) -> bool {
    if (springs.len() as i32) < len {
        return false;
    }
    for (idx, val) in springs.chars().enumerate() {
        if (idx as i32) < len {
            if val != '#' && val != '?' {
                return false;
            }
        } else {
            if val != '.' && val != '?' {
                return false;
            }
            break;
        }
    }
    return true;
}

fn d12_p1_arrangments_count(springs: &str, damaged: &Vec<i32>, offset: usize, lut: &mut HashMap<(String, usize), i64>) -> i64 {
    //println!("Checking {} {:?} {}", springs, damaged, offset);
    if lut.contains_key(&(springs.to_string(), offset)) {
        return *lut.get(&(springs.to_string(), offset)).unwrap();
    }

    if springs.is_empty() {
        if offset == damaged.len() {
            //println!("Valid chain!");
            return 1;
        }
        return 0;
    }

    if springs.chars().nth(0).unwrap() == '.' {
        return d12_p1_arrangments_count(&springs[1..], damaged, offset, lut);
    }
    else if springs.chars().nth(0).unwrap() == '?' {
        // check possible solutions when current '?' is assumed to be '.'
        //println!("Count '.' branch {}", springs);
        let mut result = d12_p1_arrangments_count(&springs[1..], damaged, offset, lut);
        // check possible solutions when current '?' is assumed to be '#'
        //println!("Count '#' branch {}", springs);
        if offset < damaged.len() && d12_p1_valid_chain(springs, damaged[offset]) {
            let skip = if (damaged[offset] as usize) < springs.len() { 1 } else { 0 };
            result += d12_p1_arrangments_count(&springs[damaged[offset] as usize + skip..], damaged, offset + 1, lut);
        }
        lut.insert((springs.to_string(), offset), result);
        return result;
    }
    else {
        let mut result = 0;
        if offset < damaged.len() && d12_p1_valid_chain(springs, damaged[offset]) {
            let skip = if (damaged[offset] as usize) < springs.len() { 1 } else { 0 };
            result = d12_p1_arrangments_count(&springs[damaged[offset] as usize + skip..], damaged, offset + 1, lut);
        }
        //println!("Invalid chain");
        lut.insert((springs.to_string(), offset), result);
        return result;
    }
}

fn day_12_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 12 part 1");
    let mut result = 0;
    for (idx, line) in input_lines.into_iter().enumerate() {
        let input: Vec<&str> = line.split(' ').collect();
        assert_eq!(input.len(), 2);
        let springs = input[0];
        let damaged: Vec<i32> = input[1].split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        println!("{}.:\tSprings: {} Damaged: {:?}", idx, springs, damaged);
        let mut lut: HashMap<(String, usize), i64> = HashMap::new();
        let tmp_result = d12_p1_arrangments_count(springs, &damaged, 0, &mut lut);
        println!("Line result: {}\n", tmp_result);
        result += tmp_result;
    }
    println!("Final result: {}", result);
}

fn day_12_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 12 part 2");
    let mut result = 0;
    for (idx, line) in input_lines.into_iter().enumerate() {
        let input: Vec<&str> = line.split(' ').collect();
        assert_eq!(input.len(), 2);
        let springs = input[0].to_string() + "?" + input[0] + "?" + input[0] + "?" + input[0] + "?" + input[0];
        let tmp_damaged: Vec<i32> = input[1].split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        let damaged: Vec<i32> = [&tmp_damaged[..], &tmp_damaged, &tmp_damaged, &tmp_damaged, &tmp_damaged].concat();
        println!("{}.:\tSprings: {} Damaged: {:?}", idx, springs, damaged);
        let mut lut: HashMap<(String, usize), i64> = HashMap::new();
        let tmp_result = d12_p1_arrangments_count(&springs, &damaged, 0, &mut lut);
        println!("Line result: {}\n", tmp_result);
        result += tmp_result;
    }
    println!("Final result: {}", result);
}

fn d13_p1_get_mirror(input: &Vec<String>, first_row: usize, last_row: usize, vertical: bool) -> usize {
    if vertical {
        for offset in 1..(last_row - first_row) {
            let tmp_row = first_row + offset;
            let mut valid = true;
            println!("First: {} Last: {} Offset: {}", first_row, last_row, offset);
            for idx in 0..min(last_row-offset - first_row, offset) {
                println!("Checking row: {} and {}", tmp_row - idx - 1, tmp_row + idx);
                for jdx in 0..input[first_row].len() {
                    if input[tmp_row - idx - 1].chars().nth(jdx).unwrap() != input[tmp_row + idx].chars().nth(jdx).unwrap() {
                        valid = false;
                        break;
                    }
                }
            }
            if valid {
                println!("Found middle at: {}", offset);
                return offset;
            }
        }
    }
    return 0;
}

fn day_13_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 13 part 1");
    let mut result = 0;
    let mut first_row = usize::MAX;
    for (idx, line) in input_lines.into_iter().enumerate() {
        if !line.is_empty() {
            first_row = if first_row == usize::MAX { idx } else { first_row };
            println!("{}.:\t{} (len:{})", idx, line, line.len());
        }
        else {
            println!("Block starting {}:{}\n", first_row, idx);
            result += d13_p1_get_mirror(input_lines, first_row, idx, true) * 100;
            result += d13_p1_get_mirror(input_lines, first_row, idx, false);
            first_row = usize::MAX;
        }
    }
    println!("Block starting {}:{}\n", first_row, input_lines.len());
    result += d13_p1_get_mirror(input_lines, first_row, input_lines.len(), true) * 100;
    result += d13_p1_get_mirror(input_lines, first_row, input_lines.len(), false);

    println!("Final result: {}", result);
}

fn day_13_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 13 part 2");
    let mut result = 0;
    for (idx, line) in input_lines.into_iter().enumerate() {
        println!("{}.:\t{}\n", idx, line);
        result += 1;
    }
    println!("Final result: {}", result);
}

fn d15_p1_hash(val: &str, parse_all: bool) -> (String, usize, char, u8) {
    let mut result = 0;
    let mut operation: char = '\0';
    let mut code: String = "".to_string();
    for c in val.chars() {
        if !parse_all && (c == '-' || c == '=') {
            operation = c;
            break;
        }
        code.push(c);
        result += c as usize;
        result *= 17;
        result %= 256;
    }
    let last_char = val.chars().last().unwrap();
    let lense = if last_char > '0' { last_char as u8 - '0' as u8 } else { last_char as u8 };
    return (code, result, operation, lense);
}

fn day_15_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 15 part 1");
    let mut result: usize = 0;
    for (idx, line) in input_lines.into_iter().enumerate() {
        println!("{}.:\t{}\n", idx, line);
        for step in line.split(',') {
            let tmp = d15_p1_hash(step, true);
            println!("Parsed step {} result {}", step, tmp.0);
            result += tmp.1 as usize;
        }
    }
    println!("Final result: {}", result);
}

fn day_15_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 15 part 2");
    let mut result = 0;
    let mut boxes: Vec<Vec<(String, u8)>> = vec![vec![]];
    boxes.resize(256, vec![]);
    for (idx, line) in input_lines.into_iter().enumerate() {
        println!("{}.:\t{}\n", idx, line);
        for step in line.split(',') {
            let (label, box_idx, operation, lense) = d15_p1_hash(step, false);
            println!("Parsed step {} result {} {} {} {}", step, label, box_idx, operation, lense);
            if operation == '=' {
                let mut update: bool = false;
                for bx in boxes[box_idx].iter_mut() {
                    if bx.0 == label {
                        bx.1 = lense;
                        update = true;
                        break;
                    }
                }
                if !update {
                    boxes[box_idx].push((label, lense));
                }
            }
            else if operation == '-' {
                for (bx_idx, bx) in boxes[box_idx].iter().enumerate() {
                    if bx.0 == label {
                        boxes[box_idx].remove(bx_idx);
                        break;
                    }
                }
            }
        }
    }
    for (idx, bx) in boxes.iter().enumerate() {
        if bx.is_empty() {
            continue;
        }
        println!("Box {}: {:?}", idx, bx);
        for (jdx, lense) in bx.iter().enumerate() {
            result += (idx+1) * (jdx+1) * lense.1 as usize;
        }
    }
    println!("Final result: {}", result);
}

#[derive(Debug, Hash, std::cmp::Eq, std::cmp::PartialEq)]
struct D16P1Next {
    position: (usize, usize),
    next_step: (i32, i32)
}

fn d16_p1_is_valid_pos(maze: &Vec<String>, next: &D16P1Next) -> bool {
    let next_x = next.position.0 as i32 + next.next_step.0;
    let next_y = next.position.1 as i32 + next.next_step.1;

    if next_x < 0 || next_x >= maze[0].len() as i32 {
        return false;
    }

    if next_y < 0 || next_y >= maze.len() as i32 {
        return false;
    }

    return true;
}

fn d16_p1_get_next_steps(maze: &Vec<String>, pos: (usize, usize), step: (i32, i32)) -> Vec<(i32, i32)> {
    let val = maze[pos.1].chars().nth(pos.0).unwrap();
    if val == '.' {
        return [step].to_vec();
    } else if val == '|' {
        if step == (1, 0) || step == (-1, 0) {
             return [(0, 1), (0, -1)].to_vec();
        } else {
            return [step].to_vec();
        }
    } else if val == '-' {
        if step == (0, 1) || step == (0, -1) {
            return [(1, 0), (-1, 0)].to_vec();
        } else {
            return [step].to_vec();
        }
    } else if val == '\\' {
        if step == (0, 1) {
            return [(1, 0)].to_vec();
        } else if step == (0, -1) {
            return [(-1, 0)].to_vec();
        } else if step == (1, 0) {
            return [(0, 1)].to_vec();
        } else if step == (-1, 0) {
            return [(0, -1)].to_vec();
        }
    } else if val == '/' {
        if step == (0, 1) {
            return [(-1, 0)].to_vec();
        } else if step == (0, -1) {
            return [(1, 0)].to_vec();
        } else if step == (1, 0) {
            return [(0, -1)].to_vec();
        } else if step == (-1, 0) {
            return [(0, 1)].to_vec();
        }
    }
    return [].to_vec();
}

fn d16_p1_parse_next_steps(maze: &Vec<String>, next: D16P1Next, next_steps: &mut Vec<D16P1Next>, visited: &mut HashSet<D16P1Next>) {
    visited.insert(D16P1Next { position: next.position, next_step: next.next_step });
    if !d16_p1_is_valid_pos(maze, &next) {
        return;
    }

    let next_pos = ((next.position.0 as i32 + next.next_step.0) as usize,
                    (next.position.1 as i32 + next.next_step.1) as usize);

    let val = maze[next_pos.1].chars().nth(next_pos.0).unwrap();
    if val == '.' {
        let next_step = D16P1Next{position: next_pos, next_step: next.next_step};
        if !visited.contains(&next_step) {
            next_steps.push(next_step);
        }
        return;
    } else if val == '|' {
        if next.next_step == (1, 0) || next.next_step == (-1, 0) {
            next_steps.push(D16P1Next { position: next_pos, next_step: (0, 1) });
            next_steps.push(D16P1Next { position: next_pos, next_step: (0, -1) });
        } else {
            next_steps.push(D16P1Next { position: next_pos, next_step: next.next_step });
        }
    } else if val == '-' {
        if next.next_step == (0, 1) || next.next_step == (0, -1) {
            next_steps.push(D16P1Next { position: next_pos, next_step: (1, 0) });
            next_steps.push(D16P1Next { position: next_pos, next_step: (-1, 0) });
        } else {
            next_steps.push(D16P1Next { position: next_pos, next_step: next.next_step });
        }
    } else if val == '\\' {
        if next.next_step == (0, 1) {
            next_steps.push(D16P1Next { position: next_pos, next_step: (1, 0) });
        } else if next.next_step == (0, -1) {
            next_steps.push(D16P1Next { position: next_pos, next_step: (-1, 0) });
        } else if next.next_step == (1, 0) {
            next_steps.push(D16P1Next { position: next_pos, next_step: (0, 1) });
        } else if next.next_step == (-1, 0) {
            next_steps.push(D16P1Next { position: next_pos, next_step: (0, -1) });
        }
    } else if val == '/' {
        if next.next_step == (0, 1) {
            next_steps.push(D16P1Next { position: next_pos, next_step: (-1, 0) });
        } else if next.next_step == (0, -1) {
            next_steps.push(D16P1Next { position: next_pos, next_step: (1, 0) });
        } else if next.next_step == (1, 0) {
            next_steps.push(D16P1Next { position: next_pos, next_step: (0, -1) });
        } else if next.next_step == (-1, 0) {
            next_steps.push(D16P1Next { position: next_pos, next_step: (0, 1) });
        }
    }
}

fn day_16_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 16 part 1");
    //let mut next_steps: Vec<D16P1Next> = Vec::from([D16P1Next{position: (0, 0), next_step: (1, 0)}]); // move right: x + 1, y + 0
    let mut next_steps: Vec<D16P1Next> = vec![];
    for next in d16_p1_get_next_steps(input_lines, (0, 0), (1, 0)).iter() {
        println!("Starting step: {:?}", next);
        next_steps.push(D16P1Next { position: (0, 0), next_step: *next });
    }
    let mut visited: HashSet<D16P1Next> = HashSet::new();
    while !next_steps.is_empty() {
        let next_step = next_steps.pop().unwrap();
        d16_p1_parse_next_steps(input_lines, next_step, &mut next_steps, &mut visited)
    }
    let mut unique_fields: HashSet<(usize, usize)> = HashSet::new();
    for pos in visited {
        unique_fields.insert(pos.position);
    }

    for row in 0..input_lines.len() {
        for col in 0..input_lines[0].len() {
            if unique_fields.contains(&(col, row)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }

    let result = unique_fields.len();

    println!("Final result: {}", result);
}

fn day_16_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 16 part 2");
    let mut result = 0;
    for (idx, line) in input_lines.into_iter().enumerate() {
        println!("{}.:\t{}", idx, line);
        result += 1;
    }
    println!("Final result: {}", result);
}

fn main() {
    let solutions = HashMap::from([
        ("d1p1".to_string(), day_1_part_1 as fn(&Vec<String>) ), // cast to let compiler know about item
        // type, no need to do it for latter items
        ("d1p2".to_string(), day_1_part_2 ),
        ("d2p1".to_string(), day_2_part_1 ),
        ("d2p2".to_string(), day_2_part_2 ),
        ("d3p1".to_string(), day_3_part_1 ),
        ("d3p2".to_string(), day_3_part_2 ),
        ("d4p1".to_string(), day_4_part_1 ),
        ("d4p2".to_string(), day_4_part_2 ),
        ("d5p1".to_string(), day_5_part_1 ),
        ("d5p2".to_string(), day_5_part_2 ),
        ("d6p1".to_string(), day_6_part_1 ),
        ("d6p2".to_string(), day_6_part_2 ),
        ("d7p1".to_string(), day_7_part_1 ),
        ("d7p2".to_string(), day_7_part_2 ),
        ("d8p1".to_string(), day_8_part_1 ),
        ("d8p2".to_string(), day_8_part_2 ),
        ("d9p1".to_string(), day_9_part_1 ),
        ("d9p2".to_string(), day_9_part_2 ),
        ("d10p1".to_string(), day_10_part_1 ),
        ("d10p2".to_string(), day_10_part_2 ),
        ("d11p1".to_string(), day_11_part_1 ),
        ("d11p2".to_string(), day_11_part_2 ),
        ("d12p1".to_string(), day_12_part_1 ),
        ("d12p2".to_string(), day_12_part_2 ),
        ("d13p1".to_string(), day_13_part_1 ),
        ("d13p2".to_string(), day_13_part_2 ),
        ("d15p1".to_string(), day_15_part_1 ),
        ("d15p2".to_string(), day_15_part_2 ),
        ("d16p1".to_string(), day_16_part_1 ),
        ("d16p2".to_string(), day_16_part_2 ),
    ]);
    if env::args().count() != 3 {
        println!("Usage: program_name day_and_part input_path");
        println!("\tday_and_part - i.e. d1p1 for day 1 part 1");
        println!("\tinput_path - path to file with input data");
        return;
    }

    let day_part = env::args().nth(1).unwrap();
    let input_path = env::args().nth(2).unwrap();
    let input = read_to_string(input_path).unwrap().lines().map(String::from).collect();

    if !solutions.contains_key(&day_part) {
        println!("Invalid day or part: should be dXpY, where X is day number, and Y is part number");
        return;
    }
    solutions.get(&day_part).unwrap()(&input);
}
