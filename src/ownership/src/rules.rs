fn make_owner_ship(s: String) -> (String, usize) {
    let str_length: usize = s.len();
    (s, str_length)
}

fn share_by_reference(s: &mut String) {
    s.push_str(" was pushed");
}

pub fn run() {
    {
        let str_1 = String::from("heap string");

        // Like shallow copy but str_1 was moved
        // Rust never 'deep' copy data
        let str_2 = str_1;

        let str_3 = str_2.clone();

        // str_1 was moved, no longer valid
        // no longer use
        println!("str_2: {}", str_2);
        println!("str_3: {}", str_3);
    }

    // heap string will be dropped when out of scopes

    {
        // alloc string
        let str_3 = String::from("was borrowed from str_3");

        // str_3 moved to str_4
        // str_4 borrow str_3
        let (str_4, len) = make_owner_ship(str_3);

        println!("str_4 {} has length {}", str_4, len);
    }

    {
        // We’re not allowed to modify something we have a reference to.
        let mut str_pointer = String::from("str_pointer");

        share_by_reference(&mut str_pointer);
        println!("{}", str_pointer);

        let mut p = String::from("This is mmutable string");

        // Slice
        let p_1 = &mut p[0..1];

        println!("{}", p_1);

        println!("{}, {}", p, p);
    }
}
