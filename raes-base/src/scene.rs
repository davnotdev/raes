pub trait Scene {
    fn run(&mut self) -> Box<dyn Scene>;
}
