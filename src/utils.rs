pub fn format_phone(phone: &str) -> String {
    let phone = phone.chars().filter(|c| c.is_digit(10)).collect::<String>();
    match phone.len() {
        10 => format!("({}) {}-{}", &phone[0..3], &phone[3..6], &phone[6..10]),
        11 => format!("{}-({}) {}-{}", &phone[0..1], &phone[1..4], &phone[4..7], &phone[7..11]),
        _ => phone
    }
}