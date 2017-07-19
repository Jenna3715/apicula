use std::fmt;
use std::fmt::Write;
use util::view::Viewable;

/// Sixteen-byte NUL-padded ASCII(?) string, used as human-readable names
/// in Nitro files.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Name(pub [u8; 16]);

impl Name {
    pub fn from_bytes(buf: &[u8]) -> Name {
        assert_eq!(buf.len(), 16);

        let mut arr = [0; 16];
        for i in 0..16 {
            arr[i] = buf[i];
        }
        Name(arr)
    }

    /// Returns an object that formats the name as a non-empty string
    /// of letters, digits, and underscores.
    pub fn print_safe(&self) -> NameSafePrinter {
        NameSafePrinter(self)
    }
}

impl Viewable for Name {
    fn size() -> usize { 16 }
    fn view(buf: &[u8]) -> Name {
        Name::from_bytes(buf)
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for &b in trim_trailing_nuls(&self.0[..]) {
            // Convert non-printable characters to periods (which is what
            // hex editors usually do).
            f.write_char(if b < 0x20 { '.' } else { b as char })?;
        }
        Ok(())
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char('"')?;
        for &b in trim_trailing_nuls(&self.0[..]) {
            for c in (b as char).escape_default() {
                f.write_char(c)?;
            }
        }
        f.write_char('"')
    }
}

/// Wrapper produces by `Name::print_safe`.
pub struct NameSafePrinter<'a>(pub &'a Name);

impl<'a> fmt::Display for NameSafePrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let trimmed = trim_trailing_nuls(&(self.0).0[..]);

        if trimmed.len() == 0 {
            f.write_char('_')?;
            return Ok(());
        }

        for &b in trimmed {
            let is_letter_or_digit =
                (b >= b'a' && b <= b'z') ||
                (b >= b'A' && b <= b'Z') ||
                (b >= b'0' && b <= b'9');
            let c = if is_letter_or_digit { b as char } else { '_' };
            f.write_char(c)?;
        }
        Ok(())
    }
}

/// Slice off any suffix of NUL bytes from the end of a slice.
fn trim_trailing_nuls(mut buf: &[u8]) -> &[u8] {
    while let Some((&0, rest)) = buf.split_last() {
        buf = rest;
    }
    buf
}
