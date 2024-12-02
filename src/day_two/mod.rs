use std::fs;

fn prepare() -> Vec<Vec<i32>>{
    let file = fs::read_to_string("src/day_two/input.txt").expect("file not found");
    let mut reports:Vec<Vec<i32>> = Vec::new();
    
    for line in file.lines() {
        
        let numbers: Vec<i32> = line.split_whitespace()
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect();

        reports.push(numbers);
    }
   
    reports
}

fn is_increasing(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[1] > w[0])
}

fn is_decreasing(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[1] < w[0])
}

fn is_safe(arr: &[i32]) -> bool {
   arr.windows(2).all(|w| (w[0] - w[1]).abs() >= 1 && (w[0] - w[1]).abs() <= 3 )
}

fn is_monotonic(arr: &[i32]) -> bool {
    is_increasing(arr) || is_decreasing(arr)
}



pub fn day_two_1() {
    let reports= prepare();

    let mut count = 0;
    
    for report in reports {
        if is_monotonic(&report) && is_safe(&report) {
             count += 1;
        }   
    }
    
    println!("day two part 1: {}", count);
    
}


fn check_report_damp(rep: &Vec<i32>) -> bool{  
        if is_monotonic(rep) && is_safe(rep) {
            return true;
        } else{           
            for i in 0..rep.len() {
                let mut new_report = rep.to_vec();
                new_report.remove(i);
                if is_monotonic(&new_report) && is_safe(&new_report) {
                   return true;
                } 
            }
        }
       false
}

pub fn day_two_2() {
    let reports = prepare();

    let mut count = 0;
    
    for report in reports {
      if  check_report_damp(&report) {count += 1}
    }

    println!("day two part 2: {}", count);
}
