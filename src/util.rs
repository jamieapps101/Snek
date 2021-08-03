use std::str::FromStr;

pub fn args_to_dims<'a>(input: &'a str) -> Option<[usize;2]> {
    let mut comma_index: usize = 0;
    let w: usize;
    let h: usize;
    for (index,c) in input.chars().enumerate() {
        if c==',' {
            comma_index = index;
        } else if !c.is_numeric() {
            return None;
        }
    }
    let (w_str,h_str) = input.split_at(comma_index);
    let h_str = h_str.trim_start_matches(',');
    if let Ok(temp) = usize::from_str(w_str) {
        w = temp;
    } else {
        return None;
    }

    if let Ok(temp) = usize::from_str(h_str) {
        h = temp;
    } else {
        return None;
    }

    return Some([w,h]);
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn args_to_dims_correct() {
        let input = "5,7";
        let ref_output = Some([5,7]);
        let output = args_to_dims(input);
        assert_eq!(output,ref_output);
    }

    #[test]
    fn args_to_dims_incorrect() {
        assert_eq!(args_to_dims("Nothing"),None);
        assert_eq!(args_to_dims("a,5"),None);
        assert_eq!(args_to_dims("6,b"),None);
        assert_eq!(args_to_dims("a,b"),None);
    }
}