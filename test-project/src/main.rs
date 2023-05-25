/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/

 struct President {
    firstname: &'static str,
    lastname: &'static str,
    term_start: u32,
    term_end: u32,
}

const PRESIDENTS: [President; 5] = [
    President { firstname: "Joe", lastname: "Biden", term_start: 2021, term_end: 2025 },
    President { firstname: "Donald", lastname: "Trump", term_start: 2017, term_end: 2021 },
    President { firstname: "Barack", lastname: "Obama", term_start: 2009, term_end: 2017 },
    President { firstname: "George W.", lastname: "Bush", term_start: 2001, term_end: 2009 },
    President { firstname: "Bill", lastname: "Clinton", term_start: 1993, term_end: 2001 },
];

fn main() {    
    let mut my_presidents=PRESIDENTS;
    my_presidents[0].firstname="Joseph";
    //PRESIDENTS[0].firstname="Joseph";

    for president in &my_presidents {
        println!("{} {} served from {} to {}.", president.firstname, president.lastname, president.term_start, president.term_end);
    }
}
