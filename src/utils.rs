pub fn make_secs(hours: String, minutes: String, seconds: String) -> u64 {
   let h = parse_number(hours);
   let m = parse_number(minutes);
   let s = parse_number(seconds);

   h*60*60 + m*60 + s
}

fn parse_number(t: String) -> u64 {
    match t.parse::<u64>() {
        Ok(num) => {
            num
        },
        Err(_) => {
            0
        }
    }
}

pub fn regulate_24(t: String) -> String {
    match t.parse::<i32>() {
        Ok(num) => {
            if num > 23 {
                "23".to_string()
            } else if num < 0 {
                "00".to_string()
            } else {
                t
            }
        },
        Err(_) => {"00".to_string()},
    }
}

pub fn regulate_60(t: String) -> String {
    match t.parse::<i32>() {
        Ok(num) => {
            if num > 59 {
                "59".to_string()
            } else if num < 0 {
                "00".to_string()
            } else {
                t
            }
        },
        Err(_) => {"00".to_string()},
    }
}