#[derive(Default, Clone)]
pub struct Overlap {
    f_id: String,
    g_id: String,
    len: i32,
    idt: f32,

    f_strand: i32,
    f_begin: i32,
    f_end: i32,
    f_len: i32,

    g_strand: i32,
    g_begin: i32,
    g_end: i32,
    g_len: i32,

    contained: String,
}

impl Overlap {
    // Immutable accessors
    pub fn f_id(&self) -> &String {
        &self.f_id
    }
    pub fn g_id(&self) -> &String {
        &self.g_id
    }
    pub fn len(&self) -> &i32 {
        &self.len
    }
    pub fn idt(&self) -> &f32 {
        &self.idt
    }

    pub fn f_strand(&self) -> &i32 {
        &self.f_strand
    }
    pub fn f_begin(&self) -> &i32 {
        &self.f_begin
    }
    pub fn f_end(&self) -> &i32 {
        &self.f_end
    }
    pub fn f_len(&self) -> &i32 {
        &self.f_len
    }

    pub fn g_strand(&self) -> &i32 {
        &self.g_strand
    }
    pub fn g_begin(&self) -> &i32 {
        &self.g_begin
    }
    pub fn g_end(&self) -> &i32 {
        &self.g_end
    }
    pub fn g_len(&self) -> &i32 {
        &self.g_len
    }

    pub fn contained(&self) -> &String {
        &self.contained
    }

    pub fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split('\t').collect();

        Self {
            f_id: parts[0].to_string(),
            g_id: parts[1].to_string(),
            len: parts[2].parse::<i32>().unwrap(),
            idt: parts[3].parse::<f32>().unwrap(),

            f_strand: if parts[4].parse::<i32>().unwrap() == 0 {
                0
            } else {
                1
            },
            f_begin: parts[5].parse::<i32>().unwrap(),
            f_end: parts[6].parse::<i32>().unwrap(),
            f_len: parts[7].parse::<i32>().unwrap(),

            g_strand: if parts[8].parse::<i32>().unwrap() == 0 {
                0
            } else {
                1
            },
            g_begin: parts[9].parse::<i32>().unwrap(),
            g_end: parts[10].parse::<i32>().unwrap(),
            g_len: parts[11].parse::<i32>().unwrap(),

            contained: parts[12].to_string(),
        }
    }

    pub fn from_paf(line: &str) -> Self {
        let parts: Vec<&str> = line.split('\t').collect();

        let mut this = Self {
            f_id: parts[0].to_string(),
            g_id: parts[5].to_string(),
            len: parts[10].parse::<i32>().unwrap(),
            idt: parts[9].parse::<f32>().unwrap() / parts[10].parse::<f32>().unwrap(),

            f_strand: 0,
            f_begin: parts[2].parse::<i32>().unwrap() + 1,
            f_end: parts[3].parse::<i32>().unwrap() + 1,
            f_len: parts[1].parse::<i32>().unwrap(),

            g_strand: 0,
            g_begin: 0,
            g_end: 0,
            g_len: parts[6].parse::<i32>().unwrap(),

            contained: "overlap".to_string(),
        };

        if parts[4] == "+" {
            this.g_strand = 0;
            this.g_begin = parts[7].parse::<i32>().unwrap() + 1;
            this.g_end = parts[8].parse::<i32>().unwrap() + 1;
        } else {
            this.g_strand = 1;
            this.g_begin = parts[8].parse::<i32>().unwrap() + 1;
            this.g_end = parts[7].parse::<i32>().unwrap() + 1;
        }

        if this.f_begin == 1 {
            this.f_begin = 0;
        }
        if this.f_end == 1 {
            this.f_end = 0;
        }

        if this.g_begin == 1 {
            this.g_begin = 0;
        }
        if this.g_end == 1 {
            this.g_end = 0;
        }

        this
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}\t{}\t{}\t{:.3}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.f_id,
            self.g_id,
            self.len,
            self.idt,
            self.f_strand,
            self.f_begin,
            self.f_end,
            self.f_len,
            self.g_strand,
            self.g_begin,
            self.g_end,
            self.g_len,
            self.contained,
        )
    }
}

#[test]
fn ovlp_line() {
    let tests = vec![(
                         "anchor/282/0_2680\tanchor/306/0_2073\t36\t1.000\t0\t2644\t2680\t2680\t0\t0\t36\t2073\toverlap",
                         (
                             "anchor/282/0_2680",
                             "anchor/306/0_2073",
                             36,
                             1.000,
                             0,
                             2644,
                             2680,
                             2680,
                             0,
                             0,
                             36,
                             2073,
                             "overlap",
                         ),
                     ),
                     (
                         "anchor148_9124\tpac7556_20928\t8327\t0.890\t0\t797\t9124\t9124\t0\t0\t8581\t20928\toverlap",
                         (
                             "anchor148_9124",
                             "pac7556_20928",
                             8327,
                             0.890,
                             0,
                             797,
                             9124,
                             9124,
                             0,
                             0,
                             8581,
                             20928,
                             "overlap",
                         ),
                     )
    ];
    for (line, expected) in tests {
        let ovlp = Overlap::new(line);
        assert_eq!(ovlp.to_string(), line);
        assert_eq!(ovlp.f_id(), expected.0);
        assert_eq!(ovlp.g_id(), expected.1);
        assert_eq!(*ovlp.len(), expected.2);
        assert_eq!(*ovlp.idt(), expected.3);
        assert_eq!(*ovlp.f_strand(), expected.4);
        assert_eq!(*ovlp.f_begin(), expected.5);
        assert_eq!(*ovlp.f_end(), expected.6);
        assert_eq!(*ovlp.f_len(), expected.7);
        assert_eq!(*ovlp.g_strand(), expected.8);
        assert_eq!(*ovlp.g_begin(), expected.9);
        assert_eq!(*ovlp.g_end(), expected.10);
        assert_eq!(*ovlp.g_len(), expected.11);
        assert_eq!(ovlp.contained(), expected.12);
    }
}

#[test]
fn paf_line() {
    let tests = vec![(
                         "long/5059/0_25030\t25030\t6400\t14738\t+\tlong/9413/0_8928\t8928\t188\t8927\t1427\t8739\t255\tcm:i:168",
                         (
                             "long/5059/0_25030",
                             "long/9413/0_8928",
                             8739,
                             0.163,
                             0,
                             6401,
                             14739,
                             25030,
                             0,
                             189,
                             8928,
                             8928,
                             "overlap"
                         ),
                     ),
    ];
    for (line, expected) in tests {
        let ovlp = Overlap::from_paf(line);
        //        assert_eq!(ovlp.to_string(), line);
        assert_eq!(ovlp.f_id(), expected.0);
        assert_eq!(ovlp.g_id(), expected.1);
        assert_eq!(*ovlp.len(), expected.2);
        assert!((*ovlp.idt() - expected.3).abs() < 0.001);
        assert_eq!(*ovlp.f_strand(), expected.4);
        assert_eq!(*ovlp.f_begin(), expected.5);
        assert_eq!(*ovlp.f_end(), expected.6);
        assert_eq!(*ovlp.f_len(), expected.7);
        assert_eq!(*ovlp.g_strand(), expected.8);
        assert_eq!(*ovlp.g_begin(), expected.9);
        assert_eq!(*ovlp.g_end(), expected.10);
        assert_eq!(*ovlp.g_len(), expected.11);
        assert_eq!(ovlp.contained(), expected.12);
    }
}
