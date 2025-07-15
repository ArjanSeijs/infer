use std::io::Write;

pub mod attr;
pub mod basetypename;
pub mod boolexp;
pub mod constant;
pub mod exp;
pub mod fielddecl;
pub mod fieldname;
pub mod global;
pub mod ident;
pub mod instr;
pub mod location;
pub mod module;
pub mod name;
pub mod node;
pub mod nodename;
pub mod procdecl;
pub mod procdesc;
pub mod procname;
pub mod qualifiedprocname;
pub mod sourcefile;
pub mod struct_;
pub mod terminator;
pub mod typ;
pub mod typename;
pub mod varname;

pub trait WriteTextual {
    fn write(&self, out: &mut impl Write) -> ();
}

pub trait PrintTextual {
    fn pp(&self) -> String;
}

pub trait PrintTextualWithSeperator {
    fn pp_list(&self, seperator: &str) -> String;
    fn pp_comma_list(&self) -> String;
}

impl<T: PrintTextual> PrintTextualWithSeperator for Vec<T> {
    fn pp_list(&self, seperator: &str) -> String {
        let mapped: Vec<String> = self.iter().map(|v| v.pp()).collect();
        mapped.join(seperator)
    }

    fn pp_comma_list(&self) -> String {
        self.pp_list(&",")
    }
}
