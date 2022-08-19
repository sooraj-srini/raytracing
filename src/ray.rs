pub mod ray {
    use geometric::Vector3;

    type Type = Vector3<f64>;

    pub struct Ray {
        origin: Type,
        dir: Vector3<f64>,
    }

    impl Ray {
        fn orig(&mut self) -> &Vector3<f64> {
            &self.origin
        }
        fn at(&mut self, t: f64) -> Vector3<f64> {
            self.origin + self.dir * t
        }
    }
}
