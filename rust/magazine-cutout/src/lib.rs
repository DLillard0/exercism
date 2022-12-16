// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map = HashMap::new();
    for &i in magazine {
        match map.get(i) {
            Some(&v) => {
                map.insert(i, v + 1);
            },
            None => {
                map.insert(i, 1);
            }  
        }
    }
    let mut flag = true;
    for &j in note {
        match map.get(j) {
            Some(&v) => {
                if v == 0 {
                    flag = false;
                    break;
                } else {
                    map.insert(j, v - 1);
                }
            },
            None => {
                flag = false;
                break;
            }            
        }
    }
    flag
}
