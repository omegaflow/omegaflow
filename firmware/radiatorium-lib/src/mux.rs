pub const MUX_A: u8 = 0x70;
pub const MUX_B: u8 = 0x71;

pub fn select(way: u8) -> Option<(u8, u8)> {
    if way >= 16 {
        return None;
    }
    let mux = if way < 8 { MUX_A } else { MUX_B };
    let control = 1u8 << (way % 8);
    Some((mux, control))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn way_zero_opens_first_channel_of_a() {
        assert_eq!(select(0), Some((MUX_A, 0x01)));
    }

    #[test]
    fn way_seven_opens_last_channel_of_a() {
        assert_eq!(select(7), Some((MUX_A, 0x80)));
    }

    #[test]
    fn way_eight_opens_first_channel_of_b() {
        assert_eq!(select(8), Some((MUX_B, 0x01)));
    }

    #[test]
    fn way_fifteen_opens_last_channel_of_b() {
        assert_eq!(select(15), Some((MUX_B, 0x80)));
    }

    #[test]
    fn way_sixteen_is_absent() {
        assert_eq!(select(16), None);
    }
}
