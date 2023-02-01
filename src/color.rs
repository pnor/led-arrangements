/// Representing Colors to assign to lights in the LED Strip

#[derive(Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }

    /// Dim this color so its brightness is `amount` percentage of what it was
    pub fn dim(&mut self, amount: f64) {
        self.red = ((self.red as f64) * amount) as u8;
        self.green = ((self.green as f64) * amount) as u8;
        self.blue = ((self.blue as f64) * amount) as u8;
    }

    /// Returns this color's components as a (r, g, b) tuple with values from 0..1
    pub fn float_components(&self) -> (f32, f32, f32) {
        let r = self.red as f32 / 255.0;
        let g = self.green as f32 / 255.0;
        let b = self.blue as f32 / 255.0;
        return (r, g, b);
    }
}
