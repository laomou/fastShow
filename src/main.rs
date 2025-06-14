slint::slint! {
    export { AppWindow } from "ui/main.slint";
}

mod file_manager;
use file_manager::FileManager;
use slint::VecModel;
use std::rc::Rc;

fn update_ui(ui: &AppWindow, fm: &FileManager) {
    ui.set_current_path(fm.get_current_path().into());
    ui.set_tree_items(Rc::new(VecModel::from(fm.get_tree_items())).into());
    ui.set_contents(Rc::new(VecModel::from(fm.get_contents())).into());
}

fn main() -> Result<(), slint::PlatformError> {
    let mut file_manager = FileManager::new();
    let ui = AppWindow::new()?;

    update_ui(&ui, &file_manager);

    ui.run()
}
