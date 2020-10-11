pub struct LadderEntry {
    pub upper: &'static str,
    pub lower: &'static str,
    pub value: u16,
}

pub static VALUES: &[LadderEntry] = &[
    LadderEntry {
        upper: "M",
        lower: "m",
        value: 1000,
    },
    LadderEntry {
        upper: "CM",
        lower: "cm",
        value: 900,
    },
    LadderEntry {
        upper: "D",
        lower: "d",
        value: 500,
    },
    LadderEntry {
        upper: "CD",
        lower: "cd",
        value: 400,
    },
    LadderEntry {
        upper: "C",
        lower: "c",
        value: 100,
    },
    LadderEntry {
        upper: "XC",
        lower: "xc",
        value: 90,
    },
    LadderEntry {
        upper: "L",
        lower: "l",
        value: 50,
    },
    LadderEntry {
        upper: "XL",
        lower: "xl",
        value: 40,
    },
    LadderEntry {
        upper: "X",
        lower: "x",
        value: 10,
    },
    LadderEntry {
        upper: "IX",
        lower: "ix",
        value: 9,
    },
    LadderEntry {
        upper: "V",
        lower: "v",
        value: 5,
    },
    LadderEntry {
        upper: "IV",
        lower: "iv",
        value: 4,
    },
    LadderEntry {
        upper: "I",
        lower: "i",
        value: 1,
    },
];
