pub mod ray {
    use geometric::Vector3;

    type Type = Vector3<f64>;

    pub struct Ray {
        pub origin: Type,
        pub dir: Type,
    }

    impl Ray {
        pub fn orig(&mut self) -> &Vector3<f64> {
            &self.origin
        }
        pub fn at(&mut self, t: f64) -> Vector3<f64> {
            self.origin + self.dir * t
        }
        pub fn direction(&self) -> Vector3<f64> {
            self.dir.clone()
        }
    }
}
