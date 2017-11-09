//#![allow(unused)]

extern crate byteorder;

#[cfg(test)]
mod test {
    use std::path::Path;
    use std::fs::File;
    use binary::*;
    //use std::io::Error;

    #[test]
    fn test_read() {
        println!("Testing binary input for grapevinerust.");
        let path = "databases/testvine.gv3";
        print!("Opening path {}...", path);
        let path = Path::new(path);
        let file = File::open(path);

        if let Err(e) = file {
            panic!("Failed opening file: {:?}", e.kind());
        }
        else if let Ok(file) = file {
            println!("Done.\n");
            let data = GameData::new(file);
            assert!(data.is_ok());
            assert!(!data.is_err());
            let data = data.unwrap();
            println!("Header is \"{}\"", data.header);
            assert!(data.header == "GVBG");
            println!("Version is \"{}\"", data.version);
            assert!(data.version == 3.0);
            println!("Size is \"{}\"", data.size);
            // We don't want to assert the size until we have a stable database.
            //assert!(data.size == 72);
            println!("Chronicle is \"{}\"", data.chronicle_title);
            assert!(data.chronicle_title == "Test Light Fortress");
            println!("Website is \"{}\"", data.website);
            assert!(data.website == "http://hunter-net.net");
            println!("Email is \"{}\"", data.email);
            assert!(data.email == "kandiyohi.snow@gmail.com");
            println!("Phone is \"{}\"", data.phone);
            assert!(data.phone == "5553159555");
            println!("Usual time is \"{}\"", data.usual_time);
            assert!(data.usual_time == "17:30");
            println!("Usual place is \"{}\"", data.usual_place);
            assert!(data.usual_place == "Behind the Great Wall");
            println!("Chronicle description is {:?}", data.description);
            assert!(
                data.description == "We are a test larp.\r\n\r\nThis is probation for C# port."
            );
            println!("Extended health is \"{}\"", data.extended_health);
            assert!(data.extended_health == true);
            println!("Enforced history is \"{}\"", data.enforce_history);
            assert!(data.enforce_history == true);
            println!("Temp bool is \"{}\"", data.temp_bool);
            assert!(data.temp_bool == false);
            println!("ST comment start tag is \"{}\"", data.st_comment_start);
            assert!(data.st_comment_start == "[ST]");
            println!("ST comment end tag is \"{}\"", data.st_comment_end);
            assert!(data.st_comment_end == "[/ST]");
            println!("Link trait maxes is \"{}\"", data.link_trait_maxes);
            assert!(data.link_trait_maxes == true);
            println!("Random trats string is \"{}\"", data.random_traits);
            assert!(data.random_traits == "7,5,3,5,5,5,5");
            println!("Menu file path is \"{}\"", data.menu_file_name);
            assert!(data.menu_file_name == "Grapevine Menus.gvm");
            println!("Initial header and chronicle information passes.");
        }
    }

    /// Test invalid header on file.
    #[test]
    fn test_read_invalid() {
        use std::error::Error;
        use std::io::ErrorKind;
        println!("Testing binary for grapevinerust.");
        let path = "databases/testvine_invalid.gv3";
        print!("Opening path {}...", path);
        let path = Path::new(path);
        let file = File::open(path);
        assert!(file.is_ok());
        let file = file.unwrap();
        let data = GameData::new(file);
        assert!(data.is_err());
        let data = data.unwrap_err();
        assert!(data.kind() == ErrorKind::InvalidData);
        assert!(data.description() == "Not a grapevine database file.");
    }
}

pub mod binary {
    use std::fs::File;
    //use std::path::Path;
    //use std::io::prelude::*;
    //use std::io::Read;
    //use std::io;
    //use std::vec::Vec;
    use std::io::Error;
    use std::io::ErrorKind;
    use byteorder::{ReadBytesExt, LE};
    // WriteBytesExt
    //use std::fmt;

    #[derive(PartialEq)]
    #[derive(Debug)]
    pub struct GameData {
        pub header: String,
        pub version: f64,
        pub size: u16,
        pub chronicle_title: String,
        pub website: String,
        pub email: String,
        pub phone: String,
        pub usual_time: String,
        pub usual_place: String,
        pub description: String,
        pub extended_health: bool,
        pub enforce_history: bool,
        pub temp_bool: bool,
        pub st_comment_start: String,
        pub st_comment_end: String,
        pub link_trait_maxes: bool,
        pub random_traits: String,
        pub menu_file_name: String,
    }

    impl GameData {
        pub fn new(mut file: File) -> Result<GameData, Error> {
            let header = file.read_string()?;
            if header != "GVBG" {
                let e = Error::new(ErrorKind::InvalidData, "Not a grapevine database file.");
                return Err(e);
            }
            let version = file.read_f64::<LE>()?;
            let size = file.read_u16::<LE>()?;
            let chronicle_title = file.read_string()?;
            let website = file.read_string()?;
            let email = file.read_string()?;
            let phone = file.read_string()?;
            let usual_time = file.read_string()?;
            let usual_place = file.read_string()?;
            let description = file.read_string()?;
            // Bools in vb6 are apparently stored in 2 bytes.
            let extended_health = file.read_u16::<LE>()? == 0xffff;
            let enforce_history = file.read_u16::<LE>()? == 0xffff;
            let temp_bool = if version == 2.396 { true } else { false };
            let mut st_comment_start = String::from("");
            let mut st_comment_end = String::from("");
            let mut link_trait_maxes = false;
            let mut random_traits = String::from("7,5,3,5,5,5,5");
            if version >= 2.397 {
                st_comment_start = file.read_string()?;
                st_comment_end = file.read_string()?;
                link_trait_maxes = file.read_u16::<LE>()? != 0;
                if version >= 2.399 {
                    random_traits = file.read_string()?;
                }
            }
            let mut menu_file_name = file.read_string()?;
            if menu_file_name == "" {
                use std;
                let path = std::env::current_exe()?;
                //path.pop();
                //path.push("Grapevine Menus.gvm");
                menu_file_name = String::from(path.to_str().unwrap());
            }
            // if load_menu {
            //  menu_set.open_menus(menu_file_name, false);
            // }
            //
            // file.read_calendar()?;
            if version >= 2.397 {
                // Experience points?
            }
            Ok(GameData {
                header,
                version,
                size,
                chronicle_title,
                website,
                email,
                phone,
                usual_time,
                usual_place,
                description,
                extended_health,
                enforce_history,
                temp_bool,
                st_comment_start,
                st_comment_end,
                link_trait_maxes,
                random_traits,
                menu_file_name,
            })
        }
    }

    /// Trait for reading forth-style string
    ///
    /// Forth-style strings are strings whose length is given first, and then that many
    /// bytes are read thereafter.
    ///
    /// This is used to extend File for syntactic consistency.
    trait ForthStringRead {
        fn read_string(&mut self) -> Result<String, Error>;
    }

    impl ForthStringRead for File {
        fn read_string(&mut self) -> Result<String, Error> {
            let nchars = self.read_u16::<LE>()?;
            let mut string = String::new();
            for _ in 0..nchars {
                let b = self.read_u8()?;
                string.push(b as char);
            }
            Ok(string)
        }
    }
}
