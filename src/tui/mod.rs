pub struct TUI {
    longest_line: usize,
    lines: Vec<Box<dyn PrintableLine>>,
}

trait PrintableLine {
    fn print(&self, tui: &TUI);
}

struct Header {
    name: String,
}
struct Footer;

impl PrintableLine for String {
    fn print(&self, tui: &TUI) {
        println!("│ {}{} │", self, " ".repeat(tui.longest_line - self.chars().count()));
    }
}

impl PrintableLine for Header {
    fn print(&self, tui: &TUI) {
        println!("╭─{}{}─╮", self.name, "─".repeat(tui.longest_line - self.name.chars().count()));
    }
}

impl PrintableLine for Footer {
    fn print(&self, tui: &TUI) {
        println!("╰─{}─╯", "─".repeat(tui.longest_line));
    }
}


impl TUI {
    pub fn new() -> TUI {
        TUI { longest_line: 0, lines: Vec::new() }
    }

    pub fn add_header(&mut self, name: String) {
        self.lines.push(Box::new(Header { name }));
    }

    pub fn add_line(&mut self, message: String) {
        self.longest_line = if message.len() > self.longest_line { message.len() } else { self.longest_line };
        self.lines.push(Box::new(message));
    }

    pub fn add_footer(&mut self) {
        self.lines.push(Box::new(Footer {}));
    }

    pub fn flush(&mut self) {
        self.lines.iter().for_each(|line| line.print(self));
        self.lines.clear();
    }
}