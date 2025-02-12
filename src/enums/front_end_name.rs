use log::error;
use std::fmt::Display;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FrontEndName {
    /// VirtualBox Qt GUI front-end
    Gui,
    /// VBoxHeadless (VRDE Server) front-end
    Headless,
    ///  VirtualBox SDL front-end
    Sdl,
    /// reserved value, used for aborting the currently running VM or session owner.
    Emergencystop,
}

impl FrontEndName {
    fn get_string(self) -> &'static str {
        match self {
            FrontEndName::Gui => "gui",
            FrontEndName::Headless => "headless",
            FrontEndName::Sdl => "sdl",
            FrontEndName::Emergencystop => "",
        }
    }
}
impl Into<String> for FrontEndName {
    fn into(self) -> String {
        self.get_string().to_string()
    }
}

impl From<&str> for FrontEndName {
    fn from(value: &str) -> Self {
        match value {
            "gui" => FrontEndName::Gui,
            "headless" => FrontEndName::Headless,
            "sdl" => FrontEndName::Sdl,
            "emergencystop" => FrontEndName::Emergencystop,
            &_ => {
                error!("Unknown FrontEndName. Name: {}", value);
                FrontEndName::Gui
            }
        }
    }
}
impl Into<&str> for FrontEndName {
    fn into(self) -> &'static str {
        let name = self.get_string();
        name.as_ref()
    }
}
impl Display for FrontEndName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self.get_string()))
    }
}
