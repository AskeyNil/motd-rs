pub trait Component {
    fn width(&self) -> usize {
        return 0;
    }

    fn print(&self, width: usize);
}
