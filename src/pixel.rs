pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Pixel {
    pub fn set_alpha(&mut self, a: u8) {
        self.alpha = a;
    }

    pub fn diff(&mut self, mut c: [u8; 3]) -> f32 {
        let a = self.to_lab(c);
        c = [self.red, self.green, self.blue];
        let b = self.to_lab(c);

        let dr = b[0] - a[0];
        let dg = b[1] - a[1];
        let db = b[2] - a[2];

        let diff = (dr * dr) + (dg * dg) + (db * db);
        diff.sqrt()
    }

    fn to_lab(&mut self, c: [u8; 3]) -> [f32; 3] {
        let mut cc: [f32; 3] = [
            c[0] as f32 / 255.0,
            c[1] as f32 / 255.0,
            c[2] as f32 / 255.0,
        ];

        //  sRGBにする場合
        // for i in 0..3 {
        //     if cc[i] > 0.04045 {
        //         cc[i] = ((cc[i] + 0.055) / 1.055).powf(2.4);
        //     } else {
        //         cc[i] /= 12.92;
        //     }
        // }

        //  XYZ
        let xa: [f32; 3] = [0.4124, 0.3576, 0.1805];
        let ya: [f32; 3] = [0.2126, 0.7152, 0.0722];
        let za: [f32; 3] = [0.0193, 0.1192, 0.9505];

        cc = [
            xa[0] * cc[0] + xa[1] * cc[1] + xa[2] * cc[2],
            ya[0] * cc[0] + ya[1] * cc[1] + ya[2] * cc[2],
            za[0] * cc[0] + za[1] * cc[1] + za[2] * cc[2],
        ];

        //  to Lab
        for i in 0..3 {
            cc[i] *= 100.0;
        }
        cc[0] /= 95.047;
        cc[1] /= 100.0;
        cc[2] /= 108.883;

        for i in 0..3 {
            if cc[i] > 0.008856 {
                cc[i] = cc[i].powf(1.0 / 3.0);
            } else {
                cc[i] = (7.787 * cc[i]) + (4.0 / 29.0);
            }
        }

        [
            116.0 * cc[1] - 16.0,
            500.0 * (cc[0] - cc[1]),
            200.0 * (cc[1] - cc[2]),
        ]
    }
}
