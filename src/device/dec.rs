use std::fmt::Display;

use crate::device::switch::SwitchInfo;

fn dec_cobs(cobs_bytes: impl AsRef<[u8]>) -> Option<Vec<u8>> {
    let mut cobs_bytes = cobs_bytes.as_ref().to_vec();
    if *cobs_bytes.last()? != 0 {
        return None;
    }

    let mut i = 0;
    loop {
        let i_val = *cobs_bytes.get(i)?;

        cobs_bytes[i] = 0;

        if i_val == 0 {
            break;
        } else {
            i += i_val as usize;
        }
    }

    Some(cobs_bytes[1..cobs_bytes.len() - 1].to_vec())
}

pub fn dec_raw(bytes: impl AsRef<[u8]>) -> Option<SwitchInfo> {
    let mut bytes = bytes.as_ref().to_vec();
    let mut info = SwitchInfo::default();

    // switch kind
    if bytes.get(0)? & 1 == 1 {
        info.kind = super::switch::SwitchKind::Analog;
    }

    Some(info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dec() {
        assert_eq!(dec_cobs(vec![01, 01, 00]), Some(vec![00]));
        assert_eq!(dec_cobs(vec![01, 01, 01, 00]), Some(vec![00, 00]));
        assert_eq!(dec_cobs(vec![01, 02, 11, 01, 00]), Some(vec![00, 11, 00]));
        assert_eq!(
            dec_cobs(vec![03, 11, 22, 02, 33, 00]),
            Some(vec![11, 22, 00, 33])
        );
    }
}
