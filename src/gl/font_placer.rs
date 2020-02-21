type Id = &'static str;
type PosX = u32;
type PosY = u32;
type DimX = u32;
type DimY = u32;

pub struct Placer {
    dims:   (DimX, DimY),
    unused: Vec<(Id, DimX, DimY)>,
    used:   Vec<(Id, PosX, PosY, DimX, DimY)>,
}

impl Placer {
    pub fn new(dims: (u32, u32), data: Vec<(Id, DimX, DimY)>) -> Self {
        Self {
            dims,
            unused: data,
            used: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        // Sort descending by height, then width.
        self.unused.sort_by(|a,b| {
            use std::cmp::Ordering::*;
           match b.1.cmp(&a.1) {
                Equal => b.0.cmp(&a.0),
                ord   => ord,
            }
        });

        self.place(0, 0, self.dims.0, self.dims.1);

        if !self.unused.is_empty() {
            let mut s: String = format!(
                "TexMap({} {}): failed to fit textures", 
                self.dims.0, 
                self.dims.1,
            );
            for (id, w, h) in self.unused.iter() {
                s.push_str(&format!(" ('{}' {} {})", id, w, h));
            }
            panic!(s);
        }
    }

    fn place(&mut self, x0: PosX, y0: PosY, x1: PosX, y1: PosY) {
        if let Some((t,w,h)) = self.pop_next_fit(x0, y0, x1, y1) {
            self.used.push((t,x0,y0,w,h));
            self.place(x0 + w, y0, x1, y0 + h);
            self.place(x0, y0 + h, x1, y1);
        }
    }
    
    fn next_fit(&mut self, x0: PosX, y0: PosY, x1: PosX, y1: PosY) 
        -> Option<usize> 
    {
        let w = x1 - x0;
        let h = y1 - y0;
        for (i,e) in self.unused.iter().enumerate() {
            if e.1 <= w && e.2 <= h {
                return Some(i);
            }
        }

        None
    }

    fn pop_next_fit(&mut self, x0: PosX, y0: PosY, x1: PosX, y1: PosY) 
        -> Option<(Id, DimX, DimY)> 
    {
        if let Some(i) = self.next_fit(x0, y0, x1, y1) {
            Some(self.unused.remove(i))
        }
        else {
            None
        }
    }

    pub fn results(self) -> Vec<(Id, PosX, PosY, DimX, DimY)> {
        self.used
    }
} 
