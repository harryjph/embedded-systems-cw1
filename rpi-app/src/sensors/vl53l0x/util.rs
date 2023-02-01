pub fn decode_timeout(register_value: u16) -> u16 {
    // format: "(LSByte * 2^MSByte) + 1"
    ((register_value & 0x00FF) << (((register_value & 0xFF00) as u16) >> 8))
        as u16
        + 1
}

pub fn encode_timeout(timeout_mclks: u16) -> u16 {
    if timeout_mclks == 0 {
        return 0;
    }
    let mut ls_byte: u32;
    let mut ms_byte: u16 = 0;

    ls_byte = (timeout_mclks as u32) - 1;

    while (ls_byte & 0xFFFFFF00) > 0 {
        ls_byte >>= 1;
        ms_byte += 1;
    }

    return (ms_byte << 8) | ((ls_byte & 0xFF) as u16);
}

pub fn calc_macro_period(vcsel_period_pclks: u8) -> u32 {
    ((2304u32 * (vcsel_period_pclks as u32) * 1655u32) + 500u32) / 1000u32
}

pub fn timeout_mclks_to_microseconds(
    timeout_period_mclks: u16,
    vcsel_period_pclks: u8,
) -> u32 {
    let macro_period_nanoseconds: u32 =
        calc_macro_period(vcsel_period_pclks) as u32;
    (((timeout_period_mclks as u32) * macro_period_nanoseconds)
        + (macro_period_nanoseconds / 2))
        / 1000
}

pub fn timeout_microseconds_to_mclks(
    timeout_period_microseconds: u32,
    vcsel_period_pclks: u8,
) -> u32 {
    let macro_period_nanoseconds: u32 =
        calc_macro_period(vcsel_period_pclks) as u32;

    ((timeout_period_microseconds * 1000) + (macro_period_nanoseconds / 2))
        / macro_period_nanoseconds
}

// Decode VCSEL (vertical cavity surface emitting laser) pulse period in PCLKs from register value based on VL53L0X_decode_vcsel_period()
pub fn decode_vcsel_period(register_value: u8) -> u8 {
    ((register_value) + 1) << 1
}
