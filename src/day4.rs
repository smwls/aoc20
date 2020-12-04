#[derive(PartialEq, Debug)]
enum HeightUnit {
    Cm(u32),
    In(u32)
}

#[derive(Debug, PartialEq)]
enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth
}

#[derive(PartialEq)]
enum PassportField {
    Byr(String),
    Iyr(String),
    Eyr(String),
    Hgt(String),
    Hcl(String),
    Ecl(String),
    Pid(String),
    Cid(String)
}

#[derive(Debug)]
struct PartialPassport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>
}

#[derive(Debug)]
struct ValidPartialPassport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>
}

#[derive(Debug)]
struct Passport {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: HeightUnit,
    hcl: String,
    ecl: EyeColor,
    pid: String,
    cid: Option<String>
}

fn mk_valid_byr(byr: &str) -> Option<u32> {
    let parsed_byr = byr.parse::<u32>().ok()?;
    if parsed_byr >= 1920 && parsed_byr <= 2002 {
        Some(parsed_byr)
    } else{
        None
    }
}

fn mk_valid_iyr(iyr: &str) -> Option<u32> {
    let parsed_iyr = iyr.parse::<u32>().ok()?;
    if parsed_iyr >= 2010 && parsed_iyr <= 2020 {
        Some(parsed_iyr)
    } else {
        None
    }
}

fn mk_valid_eyr(eyr: &str) -> Option<u32> {
    let parsed_eyr = eyr.parse::<u32>().ok()?;
    if parsed_eyr >= 2020 && parsed_eyr <= 2030 {
        Some(parsed_eyr)
    } else{
        None
    }
}

fn mk_valid_hgt(hgt: &str) -> Option<HeightUnit> {
    let has_cm = hgt.find("cm");
    let has_in = hgt.find("in");
    match (has_cm, has_in) {
        (Some(_), _) => {
            let number = hgt.strip_suffix("cm")?
                            .parse::<u32>().ok()?;
            if number >= 150 && number <= 193 {
                Some(HeightUnit::Cm(number))
            } else {
                None
            }                        
        },
        (_, Some(_)) => {
            let number = hgt.strip_suffix("in")?
                            .parse::<u32>().ok()?;
            if number >= 59 && number <= 76 {
                Some(HeightUnit::In(number))
            } else {
                None
            }  
        },
        _ => None
    }
}

fn mk_valid_hcl(hcl: &str) -> Option<String> {
    let mut hcl_chars = hcl.chars();
    match hcl_chars.next() {
        Some('#') => Some('#'),
        _ => None
    }?;
    match hcl.len() {
        7 => Some(7),
        _ => None
    }?;
    match hcl_chars.filter(|ch| {
        "0123456789abcdef".contains(*ch)
    }).count() {
        6 => Some(hcl.to_string()),
        _ => None
    }
}

fn mk_valid_ecl(ecl: &str) -> Option<EyeColor> {
    match ecl {
        "amb" => Some(EyeColor::Amb),
        "blu" => Some(EyeColor::Blu),
        "brn" => Some(EyeColor::Brn),
        "gry" => Some(EyeColor::Gry),
        "grn" => Some(EyeColor::Grn),
        "hzl" => Some(EyeColor::Hzl),
        "oth" => Some(EyeColor::Oth),
        _ => None
    }
}

fn mk_valid_pid(pid: &str) -> Option<String> {
    pid.parse::<u32>().ok()?;
    if pid.len() == 9 {
        Some(pid.to_string())
    } else {
        None
    }
}

fn to_passport(partial: ValidPartialPassport) -> Option<Passport> {
    let ValidPartialPassport{
        byr,
        iyr,
        eyr,
        hgt,
        hcl,
        ecl,
        pid,
        cid
    } = partial;
    let valid_byr = mk_valid_byr(&byr)?;
    let valid_iyr = mk_valid_iyr(&iyr)?;
    let valid_eyr = mk_valid_eyr(&eyr)?;
    let valid_hgt = mk_valid_hgt(&hgt)?;
    let valid_hcl = mk_valid_hcl(&hcl)?;
    let valid_ecl = mk_valid_ecl(&ecl)?;
    let valid_pid = mk_valid_pid(&pid)?;
    Some(Passport{
        byr: valid_byr,
        iyr: valid_iyr,
        eyr: valid_eyr,
        hgt: valid_hgt,
        hcl: valid_hcl,
        ecl: valid_ecl,
        pid: valid_pid,
        cid: cid
    })
}

fn count_valid_passports(partial_passports: Vec<PartialPassport>) -> usize {
    partial_passports.into_iter()
                .map(make_valid_partial_passport)
                .flat_map(|x| x.map(to_passport))
                .filter(|x| x.is_some())
                .count()
}

fn make_valid_partial_passport(partial: PartialPassport) -> Option<ValidPartialPassport> {
    let byr = partial.byr?;
    let iyr = partial.iyr?;
    let eyr = partial.eyr?;
    let hgt = partial.hgt?;
    let hcl = partial.hcl?;
    let ecl = partial.ecl?;
    let pid = partial.pid?;
    let cid = partial.cid;
    Some(ValidPartialPassport{byr, iyr, eyr, hgt, hcl, ecl, pid, cid})
}

fn count_valid_partial_passports(partial_list: Vec<PartialPassport>) -> usize {
    partial_list.into_iter().map(make_valid_partial_passport).filter(|x| x.is_some()).count()
}

fn parse_input(input: &str) -> Option<Vec<PartialPassport>> {
    let pports = input.split("\n\n");
    pports.map(|pp| {
        let field = pp.split(|ch| ch == ' ' || ch == '\n');
        let field_list = field.map(|f| {
            let mut field_string = f.split(':');
            let field_type = field_string.next()?;
            let field_contents = field_string.next()?.to_string();
            match field_type {
                "byr" => Some(PassportField::Byr(field_contents)),
                "iyr" => Some(PassportField::Iyr(field_contents)),
                "eyr" => Some(PassportField::Eyr(field_contents)),
                "hgt" => Some(PassportField::Hgt(field_contents)),
                "hcl" => Some(PassportField::Hcl(field_contents)),
                "ecl" => Some(PassportField::Ecl(field_contents)),
                "pid" => Some(PassportField::Pid(field_contents)),
                "cid" => Some(PassportField::Cid(field_contents)),
                _ => None
            }
        });
        let init = PartialPassport{
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None
        };
        Some(field_list.fold(init, |partial_pp, x| {
            match x {
                Some(PassportField::Byr(a)) => PartialPassport{byr: Some(a), ..partial_pp},
                Some(PassportField::Iyr(a)) => PartialPassport{iyr: Some(a), ..partial_pp},
                Some(PassportField::Eyr(a)) => PartialPassport{eyr: Some(a), ..partial_pp},
                Some(PassportField::Hgt(a)) => PartialPassport{hgt: Some(a), ..partial_pp},
                Some(PassportField::Hcl(a)) => PartialPassport{hcl: Some(a), ..partial_pp},
                Some(PassportField::Ecl(a)) => PartialPassport{ecl: Some(a), ..partial_pp},
                Some(PassportField::Pid(a)) => PartialPassport{pid: Some(a), ..partial_pp},
                Some(PassportField::Cid(a)) => PartialPassport{cid: Some(a), ..partial_pp},
                _ => partial_pp
            }
        }))
    }).collect()
}

pub fn run_a(input: &str) {
    let passports = parse_input(input).unwrap();
    let count = count_valid_partial_passports(passports);
    println!("{:?}", count);
}
pub fn run_b(input: &str) {
    let passports = parse_input(input).unwrap();
    let count = count_valid_passports(passports);
    println!("{:?}", count);
}