use std::collections::HashMap;
pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
    let mut map: HashMap<&String, i32> = HashMap::new();
    arr.iter().for_each(|str| {
        map.entry(&str).and_modify(|e| *e += 1).or_insert(1);
    });

    let mut count = 0;

    for s in arr.iter() {
        if map.get(s).unwrap() == &1 {
            count += 1;
            if count == k {
                return s.to_string();
            }
        }
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_distinct() {
        // assert_eq!(
        //     kth_distinct(
        //         vec![
        //             "d".to_string(),
        //             "b".to_string(),
        //             "c".to_string(),
        //             "b".to_string(),
        //             "c".to_string(),
        //             "a".to_string()
        //         ],
        //         2
        //     ),
        //     "a".to_string()
        // );
        // assert_eq!(
        //     kth_distinct(
        //         vec!["aaa".to_string(), "aa".to_string(), "a".to_string(),],
        //         1
        //     ),
        //     "aaa".to_string()
        // );
        // assert_eq!(
        //     kth_distinct(vec!["a".to_string(), "b".to_string(), "a".to_string(),], 3),
        //     "".to_string()
        // );

        assert_eq!(
            kth_distinct(
                vec![
                    "c".to_string(),
                    "exjk".to_string(),
                    "nbmg".to_string(),
                    "kgnas".to_string(),
                    "s".to_string(),
                    "oydx".to_string(),
                    "ghpao".to_string(),
                    "c".to_string(),
                    "r".to_string(),
                    "ohdm".to_string(),
                    "fq".to_string(),
                    "ashgg".to_string(),
                    "mm".to_string(),
                    "cc".to_string(),
                    "mymy".to_string(),
                    "w".to_string(),
                    "t".to_string(),
                    "neb".to_string(),
                    "grjdb".to_string(),
                    "cukk".to_string(),
                    "ujyhn".to_string(),
                    "dq".to_string(),
                    "hhuo".to_string(),
                    "qu".to_string(),
                    "seslw".to_string(),
                    "ybulz".to_string(),
                    "iug".to_string(),
                    "rs".to_string(),
                    "kyfu".to_string(),
                    "krz".to_string(),
                    "nw".to_string(),
                    "txnn".to_string(),
                    "r".to_string(),
                    "zpuao".to_string(),
                    "sh".to_string(),
                    "rfc".to_string(),
                    "c".to_string(),
                    "hgr".to_string(),
                    "jfia".to_string(),
                    "egm".to_string(),
                    "gmuuv".to_string(),
                    "gh".to_string(),
                    "x".to_string(),
                    "nfvgv".to_string(),
                    "ibo".to_string(),
                    "al".to_string(),
                    "wn".to_string(),
                    "o".to_string(),
                    "dyu".to_string(),
                    "zgkk".to_string(),
                    "gdzrf".to_string(),
                    "m".to_string(),
                    "ui".to_string(),
                    "xwsj".to_string(),
                    "zeld".to_string(),
                    "muowr".to_string(),
                    "d".to_string(),
                    "xgiu".to_string(),
                    "yfu".to_string()
                ],
                19
            ),
            "dq".to_string()
        );
    }
}
