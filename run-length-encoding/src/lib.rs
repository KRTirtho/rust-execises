pub fn encode(source: &str) -> String {
    let src_vec: Vec<char> = source.chars().collect::<Vec<char>>();
    let mut count: i64 = 0;
    let mut result = String::new();

    for (index, src_el) in src_vec.iter().enumerate() {
        let prev_char = if index as isize - 1 >= 0 {
            src_vec[index - 1]
        } else {
            '0'
        };
        let next_char = if index + 1 <= src_vec.len() - 1 {
            src_vec[index + 1]
        } else {
            '0'
        };
        // to detect the duplicate val
        if prev_char == *src_el || next_char == *src_el {
            count += 1;
            // this is the end of duplication, yeah save data
            if next_char != *src_el {
                result += &count.to_string();
                result.push(*src_el);
                count = 0; // resetting the value
            }
        } else if prev_char != *src_el && next_char != *src_el {
            result.push(*src_el);
            count = 0;
        }
    }
    result
}

pub fn decode(source: &str) -> String {
    let src_vec = source.chars().collect::<Vec<char>>();

    let mut result = String::new();

    let mut count = String::new();

    for src_el in src_vec.iter() {
        let src_el_as_int = match src_el.to_digit(10) {
          Some(val)=>val,
          _ => 0
        };

        let count_as_int = match count.parse::<i64>(){
          Ok(val)=>val,
          _ => 0
        };

        if src_el_as_int != 0 {
          // current src_el is a int/number
          count.push(*src_el);
        }
        else if src_el_as_int == 0 && count != ""{
          // src_el is not a int
          for _ in  0..count_as_int{
            result.push(*src_el);
          }
          // resetting co
          count = String::from("");
        }
        else{
          //default to push src_el to the result string
          result.push(*src_el);
          count = String::from("");
        }
    }

    result
}
