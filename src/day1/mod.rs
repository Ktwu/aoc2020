use arr_macro::arr;

pub fn report_repair_p1(input: &[usize]) -> Option<u64> {
    let mut cache: [bool; 2021] = [false; 2021];
    for element in input.iter() {
        cache[*element] = true;
        let partner = 2020 - element;
        if cache[partner] {
            return Some((element * partner) as u64);
        }
    }
    return None;
}

pub fn report_repair_p2(input: &[usize]) -> Option<u64> {
    let mut cache: [Vec<(usize, usize)>; 2021] = arr!(vec![]; 2021);
    // O(n^2)
    for i in 0..input.len() {
        for j in (i+1)..input.len() {
            let candidate_index = input[i] + input[j];
            if candidate_index < cache.len() {
                cache[candidate_index].push((i, j));
            }
        }
    }

    // O(n)
    for (i, element) in input.iter().enumerate() {
        let candidates = &cache[2020 - element];
        for indices in candidates.iter() {
            if indices.0 != i && indices.1 != i {
                return Some((element * input[indices.0] * input[indices.1]) as u64);
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
