use core::{iter::Peekable, result, str::Chars};

/// Parsing Error
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidChar(char),
    InvalidSequenceIo,
}

/// Sounds
enum Sound {
    M,
    P,
    F,
    Th,
    N,
    T,
    S,
    Sh,
    Q,
    K,
    X,
    Kh,
    L,
    B,
    V,
    W,
    C,
    D,
    Z,
    Zh,
    R,
    G,
    Rh,
    Gh,
    Y,
    U,
    YhIe,
    Ae,
    Ih,
    Iy,
    Ah,
    Ia,
    Eh,
    Ea,
    UhOe,
    Ou,
    Oh,
    Oa,
    Ay,
    Ai,
    Au,
    Ao,
    Ey,
    Ei,
    Eu,
    Eo,
    Iu,
    Oy,
    Oi,
    H,
}

struct Sounds<'a>(Peekable<Chars<'a>>, Option<Sound>);

impl Iterator for Sounds<'_> {
    type Item = Result<Sound>;

    fn next(&mut self) -> Option<Result<Sound>> {
        if let Some(sound) = self.1.take() {
            return Some(Ok(sound));
        }

        let this = self.0.next()?;
        let next = self.0.peek();

        Some(match (this, next) {
            ('i', Some('o')) => Err(Error::InvalidSequenceIo),
            ('c', Some('h')) => {
                self.0.next();
                self.1 = Some(Sound::Sh);
                Ok(Sound::T)
            }
            ('č', _) => {
                self.1 = Some(Sound::Sh);
                Ok(Sound::T)
            }
            ('j', _) => {
                self.1 = Some(Sound::Zh);
                Ok(Sound::D)
            }
            ('t', Some('h')) => {
                self.0.next();
                Ok(Sound::Th)
            }
            ('s', Some('h')) => {
                self.0.next();
                Ok(Sound::Sh)
            }
            ('k', Some('h')) => {
                self.0.next();
                Ok(Sound::Kh)
            }
            ('z', Some('h')) => {
                self.0.next();
                Ok(Sound::Zh)
            }
            ('r', Some('h')) => {
                self.0.next();
                Ok(Sound::Rh)
            }
            ('g', Some('h')) => {
                self.0.next();
                Ok(Sound::Gh)
            }
            ('a', Some('e')) | ('y', Some('\'') | None) => {
                self.0.next();
                Ok(Sound::Ae)
            }
            ('y', Some('a' | 'e' | 'i' | 'o' | 'u')) => {
                self.0.next();
                Ok(Sound::Y)
            }
            ('i', Some('a')) | ('a', Some('\'') | None) => {
                self.0.next();
                Ok(Sound::Ia)
            }
            ('e', Some('a')) | ('e', Some('\'') | None) => {
                self.0.next();
                Ok(Sound::Ea)
            }
            ('u', Some('a' | 'e' | 'i' | 'o' | 'u')) => {
                self.0.next();
                Ok(Sound::U)
            }
            ('o', Some('a')) | ('o', Some('\'') | None) => {
                self.0.next();
                Ok(Sound::Oa)
            }
            ('i', Some('y')) | ('i', Some('\'') | None) => {
                self.0.next();
                Ok(Sound::Iy)
            }
            ('o', Some('u')) | ('u', Some('\'') | None) => {
                self.0.next();
                Ok(Sound::Ou)
            }
            ('a', Some('y')) => {
                self.0.next();
                Ok(Sound::Ay)
            }
            ('a', Some('i')) => {
                self.0.next();
                Ok(Sound::Ai)
            }
            ('a', Some('u')) => {
                self.0.next();
                Ok(Sound::Au)
            }
            ('a', Some('o')) => {
                self.0.next();
                Ok(Sound::Ao)
            }
            ('e', Some('y')) => {
                self.0.next();
                Ok(Sound::Ey)
            }
            ('e', Some('i')) => {
                self.0.next();
                Ok(Sound::Ei)
            }
            ('e', Some('u')) => {
                self.0.next();
                Ok(Sound::Eu)
            }
            ('e', Some('o')) => {
                self.0.next();
                Ok(Sound::Eo)
            }
            ('i', Some('u')) => {
                self.0.next();
                Ok(Sound::Iu)
            }
            ('o', Some('y')) => {
                self.0.next();
                Ok(Sound::Oy)
            }
            ('o', Some('i')) => {
                self.0.next();
                Ok(Sound::Oi)
            }
            ('\u{F6000}', _) | ('m', _) => Ok(Sound::M),
            ('\u{F6001}', _) | ('p', _) => Ok(Sound::P),
            ('\u{F6002}', _) | ('f', _) => Ok(Sound::F),
            ('\u{F6003}', _) | ('þ', _) => Ok(Sound::Th),
            ('\u{F6004}', _) | ('n', _) => Ok(Sound::N),
            ('\u{F6005}', _) | ('t', _) => Ok(Sound::T),
            ('\u{F6006}', _) | ('s', _) => Ok(Sound::S),
            ('\u{F6007}', _) | ('š', _) => Ok(Sound::Sh),
            ('\u{F6008}', _) | ('q', _) | ('ŋ', _) => Ok(Sound::Q),
            ('\u{F6009}', _) | ('k', _) => Ok(Sound::K),
            ('\u{F600A}', _) | ('x', _) => Ok(Sound::X),
            ('\u{F600B}', _) | ('ǩ', _) => Ok(Sound::Kh),
            ('\u{F600C}', _) | ('l', _) => Ok(Sound::L),
            ('\u{F600D}', _) | ('b', _) => Ok(Sound::B),
            ('\u{F600E}', _) | ('v', _) => Ok(Sound::V),
            ('\u{F600F}', _) | ('w', _) => Ok(Sound::W),
            ('\u{F6010}', _) | ('c', _) => Ok(Sound::C),
            ('\u{F6011}', _) | ('d', _) => Ok(Sound::D),
            ('\u{F6012}', _) | ('z', _) => Ok(Sound::Z),
            ('\u{F6013}', _) | ('ž', _) => Ok(Sound::Zh),
            ('\u{F6014}', _) | ('r', _) => Ok(Sound::R),
            ('\u{F6015}', _) | ('g', _) => Ok(Sound::G),
            ('\u{F6016}', _) | ('ř', _) => Ok(Sound::Rh),
            ('\u{F6017}', _) | ('ǧ', _) => Ok(Sound::Gh),
            ('\u{F6018}', _) | ('y', Some(_)) => Ok(Sound::YhIe),
            ('\u{F6019}', _) => Ok(Sound::Ae),
            ('\u{F601A}', _) | ('i', Some(_)) => Ok(Sound::Ih),
            ('\u{F601B}', _) => Ok(Sound::Y),
            ('\u{F601C}', _) | ('a', Some(_)) => Ok(Sound::Ah),
            ('\u{F601D}', _) => Ok(Sound::Ia),
            ('\u{F601E}', _) | ('e', Some(_)) => Ok(Sound::Eh),
            ('\u{F601F}', _) => Ok(Sound::Ea),
            ('\u{F6020}', _) | ('u', Some(_)) => Ok(Sound::UhOe),
            ('\u{F6021}', _) => Ok(Sound::U),
            ('\u{F6022}', _) | ('o', Some(_)) => Ok(Sound::Oh),
            ('\u{F6023}', _) => Ok(Sound::Oa),
            ('\u{F6024}', _) => Ok(Sound::Ay),
            ('\u{F6025}', _) => Ok(Sound::Ai),
            ('\u{F6026}', _) => Ok(Sound::Au),
            ('\u{F6027}', _) => Ok(Sound::Ao),
            ('\u{F6028}', _) => Ok(Sound::Ey),
            ('\u{F6029}', _) => Ok(Sound::Ei),
            ('\u{F602A}', _) => Ok(Sound::Eu),
            ('\u{F602B}', _) => Ok(Sound::Eo),
            ('\u{F602C}', _) => Ok(Sound::Iu),
            ('\u{F602D}', _) => Ok(Sound::Oy),
            ('\u{F602E}', _) => Ok(Sound::Oi),
            ('\u{F602F}', _) | ('h', _) => Ok(Sound::H),
            _ => Err(Error::InvalidChar(this)),
        })
    }
}

/// Result type alias
pub type Result<T = (), E = Error> = result::Result<T, E>;

/// Convert Zoitei ASCII/Unicode/Script to ASCII.
pub fn to_ascii(text: impl AsRef<str>, output: &mut String) -> Result<&str> {
    let mut last_y = false;
    let mut last_u = false;
    let mut sounds = Sounds(text.as_ref().chars().peekable(), None).peekable();

    while let Some(sound) = sounds.next() {
        let sound = sound?;
        let next = sounds.peek();
        let is_ending = next.is_none();

        match sound {
            Sound::M => output.push('m'),
            Sound::P => output.push('p'),
            Sound::F => output.push('f'),
            Sound::Th => output.push_str("th"),
            Sound::N => output.push('n'),
            Sound::T if matches!(next, Some(Ok(Sound::Sh))) => {
                sounds.next();
                output.push_str("ch")
            }
            Sound::T => output.push('t'),
            Sound::S => output.push('s'),
            Sound::Sh => output.push_str("sh"),
            Sound::Q => output.push('q'),
            Sound::K => output.push('k'),
            Sound::X => output.push('x'),
            Sound::Kh => output.push_str("kh"),
            Sound::L => output.push('l'),
            Sound::B => output.push('b'),
            Sound::V => output.push('v'),
            Sound::W => output.push('w'),
            Sound::C => output.push('c'),
            Sound::D if matches!(next, Some(Ok(Sound::Zh))) => {
                sounds.next();
                output.push('j')
            }
            Sound::D => output.push('d'),
            Sound::Z => output.push('z'),
            Sound::Zh => output.push_str("zh"),
            Sound::R => output.push('r'),
            Sound::G => output.push('g'),
            Sound::Rh => output.push_str("rh"),
            Sound::Gh => output.push_str("gh"),
            Sound::Y => output.push('y'),
            Sound::U => output.push('u'),
            Sound::YhIe => output.push_str(if last_y { "ie" } else { "yh" }),
            Sound::Ae => output.push_str(if is_ending { "y" } else { "ae" }),
            Sound::Ih => output.push_str(if is_ending { "ih" } else { "i" }),
            Sound::Iy => output.push_str(if is_ending { "i" } else { "iy" }),
            Sound::Ah => output.push_str(if is_ending { "ah" } else { "a" }),
            Sound::Ia => output.push_str(if is_ending { "a" } else { "ia" }),
            Sound::Eh => output.push_str(if is_ending { "eh" } else { "e" }),
            Sound::Ea => output.push_str(if is_ending { "e" } else { "ea" }),
            Sound::UhOe => output.push_str(if last_u { "oe" } else { "uh" }),
            Sound::Ou => output.push_str(if is_ending { "u" } else { "ou" }),
            Sound::Oh => output.push_str(if is_ending { "oh" } else { "o" }),
            Sound::Oa => output.push_str(if is_ending { "o" } else { "oa" }),
            Sound::Ay => output.push_str("ay"),
            Sound::Ai => output.push_str("ai"),
            Sound::Au => output.push_str("au"),
            Sound::Ao => output.push_str("ao"),
            Sound::Ey => output.push_str("ey"),
            Sound::Ei => output.push_str("ei"),
            Sound::Eu => output.push_str("eu"),
            Sound::Eo => output.push_str("eo"),
            Sound::Iu => output.push_str("iu"),
            Sound::Oy => output.push_str("oy"),
            Sound::Oi => output.push_str("oi"),
            Sound::H => output.push_str("h"),
        }

        last_y = matches!(sound, Sound::Y);
        last_u = matches!(sound, Sound::U);
    }

    Ok(output)
}

/// Convert Zoitei ASCII/Unicode/Script to Unicode.
pub fn to_unicode(_text: impl AsRef<str>, _output: &mut String) -> Result {
    todo!()
}

/// Convert Zoitei ASCII/Unicode/Script to Script.
pub fn to_script(_text: impl AsRef<str>, _output: &mut String) -> Result {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii() {
        assert_eq!(to_ascii("nu'ri", &mut String::new()), Ok("nouri"));
        assert_eq!(to_ascii("dači", &mut String::new()), Ok("dachi"));
        assert_eq!(to_ascii("šeŋgoi", &mut String::new()), Ok("sheqgoi"));
        assert_eq!(to_ascii("doytsu", &mut String::new()), Ok("doytsu"));
    }
}
