pub struct State {
    logs: Vec<String>,
    popup_active: bool
}

impl State {
    pub fn new() -> Self {
        Self {
            logs: vec![],
            popup_active: false,
        }
    }
    
    pub fn add_log(&mut self, new_logs: String) {
        let mut logs: Vec<_> = new_logs.trim().split("\n").map(|n| n.to_owned()).collect();
        println!("{:?}", logs);
        self.logs.append(&mut logs)
    }
    
    pub fn get_logs(&self) -> &Vec<String> {
        &self.logs
    }
    
    pub fn open_popup(&mut self) {
        self.popup_active = !self.popup_active;
    }
}