pub struct Display {
    pub pixels: Vec<Vec<u8>>,
    pub size: (u32, u32),
}

impl Display {
    pub fn new(size: Option<(u32, u32)>) -> Self{
        let (width, height) = match size {
            Some((width, height)) => (width, height),
            None => (64, 32)
        };

        let pixels = vec![vec![0; width as usize]; height as usize];

        Display{
            pixels,
            size: (width, height),
        }
    }

    pub fn clear(&mut self) {
        for row in &mut self.pixels {
            row.fill(0);
        }
    }
}
