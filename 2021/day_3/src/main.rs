use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let nums: Vec<&str> = input.lines().collect();

    part_1(&nums);
    part_2(&nums);
    println!("done")
}

fn part_1(nums: &Vec<&str>) {
    // counts
    let mut counts = [0; 12];
    for num in nums {
        for (i, char) in num.chars().enumerate() {
            if char == '1' {
                counts[i] += 1;
            } else {
                counts[i] -= 1;
            }
        }
    }
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for count in counts {
        if count > 0 {
            gamma += "1";
            epsilon += "0";
        } else {
            gamma += "0";
            epsilon += "1";
        }
    }

    let num1 = u32::from_str_radix(&gamma, 2).unwrap();
    let num2 = u32::from_str_radix(&epsilon, 2).unwrap();
    // println!("{:?}, {:?}", num1, num2);

    println!("{:?}", num1 * num2)
}

fn part_2(nums: &Vec<&str>) {
    let ascii_0 = 48u8;
    let ascii_1 = 49u8;

    let mut i = 0;
    let mut nums1 = nums.clone();
    loop {
        // first get which bit is more common in i position
        let top_bit = if nums1.iter().fold(0, |acc, num| {
            if num.as_bytes()[i] == ascii_1 {
                acc + 1
            } else {
                acc - 1
            }
        }) >= 0
        {
            ascii_1
        } else {
            ascii_0
        };

        // filter nums that have the more common bit in it's position from remaining values
        // while nums1.len() > 1 {
        //     if nums1[0].as_bytes()[i] == top_bit {

        //     }
        //     nums1.pop_front();
        // }
        let nums_temp: Vec<&str> = nums1
            .iter()
            .filter(|&&x| x.as_bytes()[i] == top_bit)
            .map(|&a| a)
            .collect();

        println!("len: {:?}", nums_temp.len());

        match nums_temp.len() {
            0 => {
                nums1 = vec![nums1[nums1.len() - 1]];
                break;
            }
            1 => {
                nums1 = nums_temp;
                break;
            }
            _ => {
                nums1 = nums_temp;
                i += 1;
            }
        }
    }

    i = 0;
    let mut nums2 = nums.clone();
    loop {
        // first get which bit is more common in i position
        let least_bit = if nums2.iter().fold(0, |acc, num| {
            if num.as_bytes()[i] == ascii_1 {
                acc + 1
            } else {
                acc - 1
            }
        }) <= 0
        {
            ascii_1
        } else {
            ascii_0
        };

        // filter nums that have the more common bit in it's position from remaining values
        let nums_temp: Vec<&str> = nums2
            .iter()
            .filter(|x| x.as_bytes()[i] == least_bit)
            .map(|a| *a)
            .collect();
        match nums_temp.len() {
            0 => {
                nums2 = vec![nums2[nums2.len() - 1]];
                break;
            }
            1 => {
                nums2 = nums_temp;
                break;
            }
            _ => {
                nums2 = nums_temp;
                i += 1;
            }
        }
        println!("least: {:?}", least_bit);
        println!("{:?}: {:?}", i, nums2);
    }
    println!("dude {:?}, {:?}", nums1, nums2);
    println!(
        "{:?}",
        u32::from_str_radix(&nums1[nums1.len() - 1].chars().collect::<String>(), 2).unwrap()
            * u32::from_str_radix(&nums2[0].chars().collect::<String>(), 2).unwrap()
    );
}
