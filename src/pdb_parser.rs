// PDB File ATOM line format
//
// COLUMNS        DATA  TYPE    FIELD        DEFINITION
// -------------------------------------------------------------------------------------
//  1 -  6        Record name   "ATOM  "
//  7 - 11        Integer       serial       Atom  serial number.
// 13 - 16        Atom          name         Atom name.
// 17             Character     altLoc       Alternate location indicator.
// 18 - 20        Residue name  resName      Residue name.
// 22             Character     chainID      Chain identifier.
// 23 - 26        Integer       resSeq       Residue sequence number.
// 27             AChar         iCode        Code for insertion of residues.
// 31 - 38        Real(8.3)     x            Orthogonal coordinates for X in Angstroms.
// 39 - 46        Real(8.3)     y            Orthogonal coordinates for Y in Angstroms.
// 47 - 54        Real(8.3)     z            Orthogonal coordinates for Z in Angstroms.
// 55 - 60        Real(6.2)     occupancy    Occupancy.
// 61 - 66        Real(6.2)     tempFactor   Temperature  factor.
// 77 - 78        LString(2)    element      Element symbol, right-justified.
// 79 - 80        LString(2)    charge       Charge  on the atom.

pub struct ATOM {
    pub serial: i32,
    pub name: String,
    pub alt_loc: u8,
    pub res_name: String,
    pub chain_id: u8,
    pub res_seq: i32,
    pub icode: u8,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub occupancy: f64,
    pub temp_factor: f64,
    pub element: String,
    pub charge: String,
}

impl ATOM {
    pub fn new() -> ATOM {
        ATOM {
            serial: 0_i32,
            name: String::with_capacity(4),
            alt_loc: 0_u8,
            res_name: String::with_capacity(3),
            chain_id: 0_u8,
            res_seq: 0_i32,
            icode: 0_u8,
            x: 0_f64,
            y: 0_f64,
            z: 0_f64,
            occupancy: 0_f64,
            temp_factor: 0_f64,
            element: String::with_capacity(2),
            charge: String::with_capacity(2),
        }
    }
}