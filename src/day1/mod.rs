pub fn report_repair_p1(input: &[usize]) -> Option<u64> {
    let mut cache: [bool; 2021] = [false; 2021];
    for element in input.iter() {
        let partner = 2020 - element;
        if cache[partner] {
            return Some((element * partner) as u64);
        }
        cache[*element] = true;
    }
    return None;
}

pub fn report_repair_p2(input: &[usize]) -> Option<u64> {
    let mut count_cache: [u8; 2021] = [0; 2021];

    // Cases
    // 3*x = 2020
    // 2*x + y = 2020
    // x + y + z = 2020
    //
    // The first situation is impossible, as 2020 is not cleanly divisible by 3.
    // We consider the other two cases below.
    for element in input.iter() {
        let mut count = count_cache[*element];
        if count < 2 {
            count_cache[*element] = count + 1;
        }
    }

    // O(n^2)
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            let candidate_sum = input[i] + input[j];
            if candidate_sum <= 2020 {
                let candidate_input = 2020 - candidate_sum;
                let required_count = if candidate_input == input[j] || candidate_input == input[i] {
                    2
                } else {
                    1
                };
                if count_cache[candidate_input] >= required_count {
                    return Some((candidate_input * input[i] * input[j]) as u64);
                }
            }
        }
    }

    return None;
}

#[test]
fn basic_report_repair() {
    assert_eq!(
        report_repair_p1(&[1721, 979, 366, 299, 675, 1456]),
        Some(514579)
    );

    assert_eq!(
        report_repair_p2(&[1721, 979, 366, 299, 675, 1456]),
        Some(241861950)
    );
}
