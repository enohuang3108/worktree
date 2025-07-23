use owo_colors::OwoColorize;

pub fn init_colors() {
    // 自動偵測終端是否支援顏色
    if !atty::is(atty::Stream::Stdout) {
        // 如果不是 TTY，可以考慮禁用顏色
    }
}

pub trait ColorizeExt {
    fn success(&self) -> String;
    fn error(&self) -> String;
    fn info(&self) -> String;
    fn warning(&self) -> String;
}

impl ColorizeExt for str {
    fn success(&self) -> String {
        self.green().to_string()
    }

    fn error(&self) -> String {
        self.red().to_string()
    }

    fn info(&self) -> String {
        self.blue().to_string()
    }

    fn warning(&self) -> String {
        self.yellow().to_string()
    }
}