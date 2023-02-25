use crate::{
    build::Target,
    molfile::{ChiralFlag, Counts, Header, MoleculeName, Parameters, Version},
    primitive::{FixedCount, FixedInteger, FixedReal, Line, Sequence},
};

use super::{Error, Reader};

pub fn header(reader: &mut Reader) -> Result<Header, Error> {
    let molecule_name =
        reader.read_line(Target::Builder(MoleculeName::start()))?;

    let parameters = if reader.has_blank() {
        reader.next_line()?;

        None
    } else {
        let user_initials = reader.read(Target::Builder(Sequence::start()))?;
        let program_name = reader.read(Target::Builder(Sequence::start()))?;
        let timestamp = reader.read(Target::Builder(Sequence::start()))?;
        let dimensional_codes =
            reader.read(Target::Builder(Sequence::start()))?;
        let major_scaling =
            reader.read(Target::Builder(FixedInteger::start()))?;
        let minor_scaling = reader.read(Target::Builder(FixedReal::start()))?;
        let energy = reader.read(Target::Builder(FixedReal::start()))?;
        let registry_number =
            reader.read_line(Target::Builder(FixedInteger::start()))?;

        Some(Parameters {
            user_initials,
            program_name,
            timestamp,
            dimensional_codes,
            major_scaling,
            minor_scaling,
            energy,
            registry_number,
        })
    };

    let comment = reader.read_line(Target::Builder(Line::start()))?;
    let atoms = reader.read(Target::Builder(FixedCount::start()))?;
    let bonds = reader.read(Target::Builder(FixedCount::start()))?;
    let atom_lists = reader.read(Target::Builder(FixedCount::start()))?;

    reader.read(Target::Builder(Sequence::<3>::start()))?;

    let chiral = reader.read(Target::Builder(ChiralFlag::start()))?;

    // sss, xxx, rrr, ppp, iii, mmm
    reader.read(Target::Builder(Sequence::<18>::start()))?;

    let version = reader.read_line(Target::Builder(Version::start()))?;

    Ok(Header {
        molecule_name,
        parameters,
        comment,
        counts: Counts {
            atoms,
            bonds,
            atom_lists,
            chiral,
            version,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn eof() {
        let mut bytes = b"".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(header(&mut reader), Err(Error::Eof(0)))
    }

    #[test]
    fn eof_after_user_initials() {
        let mut bytes = vec!["", "AB"].join("\n").into_bytes().into_iter();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(header(&mut reader), Err(Error::Eof(1)))
    }

    #[test]
    fn eof_after_program_name() {
        let mut bytes =
            vec!["", "AB-CTCORE-"].join("\n").into_bytes().into_iter();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(header(&mut reader), Err(Error::Eof(1)))
    }

    #[test]
    fn eof_after_timestamp() {
        let mut bytes = vec!["", "AB-CTCORE-0102030405"]
            .join("\n")
            .into_bytes()
            .into_iter();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(header(&mut reader), Err(Error::Eof(1)))
    }
    #[test]
    fn eof_after_dimensional_codes() {
        let mut bytes = vec!["", "AB-CTCORE-01020304052D"]
            .join("\n")
            .into_bytes()
            .into_iter();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(header(&mut reader), Err(Error::Eof(1)))
    }

    #[test]
    fn eof_after_major_scaling_factor() {
        let mut bytes = vec!["", "AB-CTCORE-01020304052D42"]
            .join("\n")
            .into_bytes()
            .into_iter();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(header(&mut reader), Err(Error::Eof(1)))
    }
    #[test]
    fn eof_after_minor_scaling_factor() {
        let mut bytes = vec!["", "AB-CTCORE-01020304052D42   1.23456"]
            .join("\n")
            .into_bytes()
            .into_iter();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(header(&mut reader), Err(Error::Eof(1)))
    }

    #[test]
    fn eof_after_energy() {
        let mut bytes =
            vec!["", "AB-CTCORE-01020304052D42   1.23456     1.23456"]
                .join("\n")
                .into_bytes()
                .into_iter();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(header(&mut reader), Err(Error::Eof(1)))
    }

    #[test]
    fn eof_after_comment() {
        let mut bytes = vec![
            "Name",
            "AB-CTCORE-01020304052D42   1.23456     1.23456 12345",
            "COMMENT",
        ]
        .join("\n")
        .into_bytes()
        .into_iter();

        let mut reader = Reader::new(&mut bytes);

        assert_eq!(header(&mut reader), Err(Error::Eof(2)))
    }

    #[test]
    #[rustfmt::skip]
    fn parameters_blank_valid() {
        let mut bytes = vec![
            "Name",
            "",
            "Comment",
           //aaabbblllfffcccsssxxxrrrpppiiimmmvvvvvv
            "  0  0  0     1                   V3000",
            ""
        ]
        .join("\n")
        .into_bytes()
        .into_iter();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            header(&mut reader),
            Ok(Header {
                molecule_name: MoleculeName::from_str("Name").unwrap(),
                parameters: None,
                comment: Line::from_str("Comment").unwrap(),
                counts: Counts {
                    atoms: FixedCount::Zero,
                    bonds: FixedCount::Zero,
                    atom_lists: FixedCount::Zero,
                    chiral: ChiralFlag::Chiral,
                    version: Version::V3,
                }
            })
        )
    }
}
