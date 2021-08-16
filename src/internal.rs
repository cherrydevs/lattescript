pub fn into_int(int: &String) -> i32 {
        return int.parse::<i32>().unwrap();
}
pub fn check_str_int(int: &String) -> bool {
    match &int[..] {
        "0" => return true,
        "1" => return true,
        "2" => return true,
        "3" => return true,
        "4" => return true,
        "5" => return true,
        "6" => return true,
        "7" => return true,
        "8" => return true,
        "9" => return true,
        _ => return false
    }
}

pub fn check_str_return_num(int: &String) -> bool {
    match &int[..] {
        int if int.contains("0") => return true,
        int if int.contains("1") => return true,
        int if int.contains("2") => return true,
        int if int.contains("3") => return true,
        int if int.contains("4") => return true,
        int if int.contains("5") => return true,
        int if int.contains("6") => return true,
        int if int.contains("7") => return true,
        int if int.contains("8") => return true,
        int if int.contains("9") => return true,
        _ => return false
    }
}