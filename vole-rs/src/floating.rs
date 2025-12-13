/// An eight bit floating point value.
pub struct Floating {
    /// The bits making up the sign bit, the exponent and the mantissa.
    pub value: u8,
}

impl Floating {
    /// Gets the value of the sign bit.
    pub fn sign_bit(&self) -> u8 {
        self.value >> 7
    }

    /// Gets the value of the exponent.
    pub fn exponent(&self) -> i8 {
        let bits = (self.value & 0x70) >> 4;
        match bits {
            0x07 => 3,
            0x06 => 2,
            0x05 => 1,
            0x04 => 0,
            0x03 => -1,
            0x02 => -2,
            0x01 => -3,
            0x00 => -4,
            _ => panic!("unsupported exponent value"),
        }
    }

    /// Gets the mantissa.
    pub fn mantissa(&self) -> u8 {
        self.value & 0x0F
    }

    /// Decodes the [Floating] into an [f32].
    pub fn decode(&self) -> f32 {
        let sign = self.sign_bit();
        let mantissa = self.mantissa();
        let exponent = self.exponent();
        let float_part_len = 4 - exponent;
        let int_part = if exponent <= 0 {
            0
        } else {
            mantissa >> float_part_len
        };

        let float_part = {
            let shifts = 8 - float_part_len;
            let float_bits = (mantissa << shifts) >> shifts;
            let mut sum = 0.0;
            for i in 1..=float_part_len {
                let devisor = (0x01 << i) as f32;
                let factor = 1.0 / devisor;
                let bit = (float_bits >> (float_part_len - i)) & 0x01;
                let v = factor * bit as f32;
                sum += v;
            }
            sum
        };

        let abs = int_part as f32 + float_part;
        if sign == 1 { -abs } else { abs }
    }

    /// Encodes an [f32] into a [Floating].
    pub fn encode(value: f32) -> Floating {
        let sign = if value < 0.0 { 1 } else { 0 };
        let int_value = (value as i8).abs();
        let fract_value = value.fract().abs();

        let exponent_value = if int_value >= 4 {
            0x07 // 3
        } else if int_value >= 2 {
            0x06 // 2
        } else if int_value >= 1 {
            0x05 // 1
        } else if fract_value >= 0.5 {
            0x04 // 0
        } else if fract_value >= 0.25 {
            0x03 // -1
        } else if fract_value >= 0.125 {
            0x02 // -2
        } else if fract_value >= 0.0625 {
            0x01 // -3
        } else if fract_value >= 0.03125 || int_value == 0 {
            0x00 // -4
        } else {
            panic!("unrepresentable")
        };

        let mantissa = {
            let mut bits: u8 = 0b00000000;
            let mut running_i = int_value;
            let mut running_f = fract_value;
            let mut shift = 7;
            if running_i >= 4 {
                bits |= 0b10000000;
                running_i -= 4;
                shift = 0;
            }
            if running_i >= 2 {
                bits |= 0b01000000;
                running_i -= 2;
                shift = i32::min(shift, 1);
            }
            if running_i >= 1 {
                bits |= 0b00100000;
                shift = i32::min(shift, 2);
            }
            if running_f >= 0.5 {
                bits |= 0b00010000;
                running_f -= 0.5;
                shift = i32::min(shift, 3);
            }
            if running_f >= 0.25 {
                bits |= 0b00001000;
                running_f -= 0.25;
                shift = i32::min(shift, 4);
            }
            if running_f >= 0.125 {
                bits |= 0b00000100;
                running_f -= 0.125;
                shift = i32::min(shift, 5);
            }
            if running_f >= 0.0625 {
                bits |= 0b00000010;
                running_f -= 0.0625;
                shift = i32::min(shift, 6);
            }
            if running_f >= 0.03125 {
                bits |= 0b00000001;
                shift = i32::min(shift, 7);
            }
            (bits << shift) >> 4
        };

        let r = sign << 7;
        let exponent_bits = exponent_value << 4;
        let r = (r | exponent_bits) | mantissa;
        Floating { value: r }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sign_bit_works() {
        let f = Floating { value: 0x80 };
        assert_eq!(f.sign_bit(), 1);

        let f = Floating { value: 0x70 };
        assert_eq!(f.sign_bit(), 0);
    }

    #[test]
    pub fn exponent_works() {
        let f = Floating { value: 0x70 };
        assert_eq!(f.exponent(), 3);
    }

    #[test]
    pub fn mantissa_works() {
        let f = Floating { value: 0x0A };
        assert_eq!(f.mantissa(), 10);
    }

    #[test]
    pub fn decode_works() {
        let f = Floating { value: 0b01101011 };
        assert_eq!(f.decode(), 2.75);

        let f = Floating { value: 0b00111100 };
        assert_eq!(f.decode(), 0.375);

        let f = Floating { value: 0b01001010 };
        assert_eq!(f.decode(), 0.625);

        let f = Floating { value: 0b01101101 };
        assert_eq!(f.decode(), 3.25);

        let f = Floating { value: 0b00111001 };
        assert_eq!(f.decode(), 0.28125);

        let f = Floating { value: 0b11011100 };
        assert_eq!(f.decode(), -1.5);

        let f = Floating { value: 0b10101011 };
        assert_eq!(f.decode(), -0.171875);

        let f = Floating { value: 0b00000000 };
        assert_eq!(f.decode(), 0.0);
    }

    #[test]
    pub fn encode_works() {
        let f = Floating::encode(-1.125);
        assert_eq!(f.value, 0b11011001);
        assert_eq!(f.decode(), -1.125);

        let f = Floating::encode(1.125);
        assert_eq!(f.value, 0b01011001);
        assert_eq!(f.decode(), 1.125);

        let f = Floating::encode(0.125);
        assert_eq!(f.exponent(), -2);
        assert_eq!(f.decode(), 0.125);

        let f = Floating::encode(0.525);
        assert_eq!(f.exponent(), 0);
        assert_eq!(f.decode(), 0.5);

        let f = Floating::encode(0.375);
        assert_eq!(f.value, 0b00111100);
        assert_eq!(f.decode(), 0.375);

        let f = Floating::encode(2.75);
        assert_eq!(f.value, 0b01101011);

        let f = Floating::encode(5.25);
        assert_eq!(f.value, 0b01111010);

        let f = Floating::encode(0.75);
        assert_eq!(f.value, 0b01001100);

        let f = Floating::encode(-3.5);
        assert_eq!(f.value, 0b11101110);

        let f = Floating::encode(-4.375);
        assert_eq!(f.value, 0b11111000);

        let f = Floating::encode(0.0);
        assert_eq!(f.value, 0b00000000);
    }
}
