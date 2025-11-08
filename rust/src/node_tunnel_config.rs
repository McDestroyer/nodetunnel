use godot::builtin::GString;
use godot::classes::{IResource, Resource};
use godot::obj::Base;
use godot::register::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct NodeTunnelConfig {
    base: Base<Resource>,

    #[export]
    pub relay_address: GString,

    #[export]
    pub app_id: GString,

    #[export]
    pub http_wakeup_enabled: bool,

    #[export]
    pub http_address: GString,

    #[export]
    pub http_wakeup_timeout: u32,
}

#[godot_api]
impl IResource for NodeTunnelConfig {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            relay_address: "127.0.0.1:8080".into(),
            http_wakeup_enabled: false,
            http_address: "".into(),
            http_wakeup_timeout: 10,
            app_id: "".into()
        }
    }
}