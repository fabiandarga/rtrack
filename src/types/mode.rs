#[derive(Debug, PartialEq, Clone)]
pub enum Mode {
    Add,
    ShowLast,
    Search,
    Track,
    Display,
    Stop,
    Quit,
    None,
}
