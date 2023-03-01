pub struct Rectangle {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl Rectangle {
    pub fn area(&self) -> f32 {
        (self.x2 - self.x1) * (self.y2 - self.y1)
    }

    pub fn intersect(&self, other: &Rectangle) -> f32 {
        let x1 = self.x1.max(other.x1);
        let y1 = self.y1.max(other.y1);
        let x2 = self.x2.min(other.x2);
        let y2 = self.y2.min(other.y2);

        if x1 >= x2 || y1 >= y2 {
            return 0.0;
        }

        (x2 - x1) * (y2 - y1)
    }
}

pub fn iou(r1: &Rectangle, r2: &Rectangle) -> f32 {
    let intersection = r1.intersect(r2) as f32;
    let union = (r1.area() + r2.area() - intersection) as f32;
    intersection / union
}