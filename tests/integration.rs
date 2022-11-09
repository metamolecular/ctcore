use std::io::Read;
use std::{fs, io};

use ctcore::read::Reader;
use ctcore::text::{Character, FortranFloat, FortranInt, Sequence, FixedCount};

use pretty_assertions::assert_eq;

const BLACKLIST: &[&[Character]] = &[
    &[Character::Dollar, Character::M, Character::D, Character::L],
    &[Character::Dollar, Character::R, Character::X, Character::N],
    &[
        Character::Dollar,
        Character::R,
        Character::D,
        Character::F,
        Character::I,
        Character::L,
        Character::E,
    ],
    &[
        Character::Dollar,
        Character::Dollar,
        Character::Dollar,
        Character::Dollar,
    ],
];

#[test]
fn read_molfile_header() -> Result<(), io::Error> {
    let file = fs::File::open("./tests/data/v3k.mol").unwrap();

    // https://stackoverflow.com/questions/26368288
    let mut err = Ok(());
    let mut buffer =
        io::BufReader::new(file)
            .bytes()
            .scan(&mut err, |err, res| match res {
                Ok(i) => Some(i),
                Err(e) => {
                    **err = Err(e);
                    None
                }
            });
    let mut reader = Reader::new(&mut buffer);

    assert_eq!(
        reader.line_with_blacklist::<80>(BLACKLIST),
        Ok(Sequence::from_str(
            format!("{: <80}", "Molecule Name").as_str()
        ))
    );
    assert_eq!(reader.newline(), Ok(()));
    assert_eq!(reader.sequence::<2>(), Ok(Sequence::from_str("AB")));
    assert_eq!(reader.sequence::<8>(), Ok(Sequence::from_str("TESTING1")));
    assert_eq!(
        reader.sequence::<10>(),
        Ok(Sequence::from_str("1103210012"))
    );
    assert_eq!(reader.sequence::<2>(), Ok(Sequence::from_str("2D")));
    assert_eq!(reader.fortran_int::<2>(), Ok(FortranInt::from_int(1)));
    assert_eq!(
        reader.fortran_float::<4, 5>(),
        Ok(FortranFloat::from_float(1234.12341))
    );
    assert_eq!(
        reader.fortran_float::<6, 5>(),
        Ok(FortranFloat::from_float(123456.12341))
    );
    assert_eq!(reader.fortran_int::<6>(), Ok(FortranInt::from_int(123456)));
    assert_eq!(reader.newline(), Ok(()));
    assert_eq!(reader.line::<80>(), Ok(Sequence::from_str("A Comment")));
    assert_eq!(reader.newline(), Ok(()));
    assert_eq!(reader.fixed_count::<3>(), Ok(FixedCount::from_int(6)));
    assert_eq!(reader.fixed_count::<3>(), Ok(FixedCount::from_int(5)));
    assert_eq!(reader.fixed_count::<3>(), Ok(FixedCount::from_int(0)));
    assert_eq!(reader.fixed_count::<3>(), Ok(None));
    assert_eq!(reader.fixed_count::<3>(), Ok(FixedCount::from_int(1)));
    assert_eq!(reader.fixed_count::<3>(), Ok(None));
    assert_eq!(reader.fixed_count::<3>(), Ok(None));
    assert_eq!(reader.fixed_count::<3>(), Ok(None));
    assert_eq!(reader.fixed_count::<3>(), Ok(None));
    assert_eq!(reader.fixed_count::<3>(), Ok(None));
    assert_eq!(reader.fixed_count::<3>(), Ok(FixedCount::from_int(999)));
    assert_eq!(reader.sequence::<6>(), Ok(Sequence::from_str(" V2000")));

    err
}
