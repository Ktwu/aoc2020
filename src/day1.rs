#![allow(dead_code)]

pub fn day1() {
    let report_repair_input = [
        1293, 1207, 1623, 1675, 1842, 1410, 85, 1108, 557, 1217, 1506, 1956, 1579, 1614, 1360,
        1544, 1946, 1666, 1972, 1814, 1699, 1778, 1529, 2002, 1768, 1173, 1407, 1201, 1264, 1739,
        1774, 1951, 1980, 1428, 1381, 1714, 884, 1939, 1295, 1694, 1168, 1971, 1352, 1462, 1828,
        1402, 1433, 1542, 1144, 1331, 1427, 1261, 1663, 1820, 1570, 1874, 1486, 1613, 1769, 1721,
        1753, 1142, 1677, 2010, 1640, 1465, 1171, 534, 1790, 2005, 1604, 1891, 1247, 1281, 1867,
        1403, 2004, 1668, 1416, 2001, 1359, 686, 1965, 1728, 1551, 1565, 1128, 1832, 1757, 1350,
        1808, 1711, 1799, 1590, 1989, 1547, 1140, 1905, 1368, 1179, 1902, 1473, 1908, 1859, 1257,
        1394, 1244, 1800, 1695, 1731, 1474, 1781, 1885, 1154, 1990, 1929, 1193, 1302, 1831, 1226,
        1418, 1400, 1435, 1645, 1655, 1843, 1227, 1481, 1754, 1290, 1685, 1498, 71, 1286, 1137,
        1288, 1758, 1987, 1471, 1839, 1545, 1682, 1615, 1475, 1849, 1985, 1568, 1795, 1184, 1863,
        1362, 1271, 1802, 1944, 1821, 1880, 1788, 1733, 1150, 1314, 1727, 1434, 1833, 1312, 1457,
        160, 1629, 1967, 1505, 1239, 1266, 1838, 1687, 1630, 1591, 1893, 1450, 1234, 1755, 1523,
        1533, 1499, 1865, 1725, 1444, 1517, 1167, 1738, 1519, 1263, 1901, 1627, 1644, 1771, 1812,
        1270, 1497, 1707, 1708, 1396,
    ];

    // 793524
    println!(
        "report_repair part 1: {:?}",
        report_repair_p1(&report_repair_input)
    );

    // 61515678
    println!(
        "report_repair part 2: {:?}",
        report_repair_p2(&report_repair_input)
    );
}

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
        let count = count_cache[*element];
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
