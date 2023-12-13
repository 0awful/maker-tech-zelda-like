use godot::engine::Control;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct InventoryGui {
    #[base]
    base: Base<Control>,
    #[init(default = false)]
    pub is_open: bool,
}

#[godot_api]
impl InventoryGui {
    #[signal]
    fn opened();
    #[signal]
    fn closed();

    #[func]
    pub fn open(&mut self) {
        self.base.set_visible(true);
        self.is_open = true;
        self.base.emit_signal("opened".into(), &[]);
    }
    #[func]
    pub fn close(&mut self) {
        self.is_open = false;
        self.base.set_visible(false);
        self.base.emit_signal("closed".into(), &[]);
    }
    #[func]
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
        self.base.set_visible(self.is_open);
        if self.is_open {
            self.base.emit_signal("opened".into(), &[]);
        } else {
            self.base.emit_signal("closed".into(), &[]);
        }
    }
}
