use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Deserialize, Serialize)]
pub enum Enum {
    #[default]
    All,
    Active,
    Completed,
}

#[derive(Deserialize, Serialize)]
pub struct TodoItem {
    pub title: String,
    pub active: bool,
    pub edit: bool,
}

impl TodoItem {
    pub fn sigleitem(&mut self, ui: &mut eframe::egui::Ui) -> bool {
        let mut change = false;
        // 创建一个水平 UI 布局
        ui.horizontal(|ui| {
            if self.edit {
                // 编辑状态下, 使用单行文本编辑框显示 `title`
                let respon = ui.text_edit_singleline(&mut self.title);
                
                // 请求文本编辑框获得焦点
                respon.request_focus();
                // 如果编辑状态下点击了 "X" 按钮, 标记 `change` 为 `true` 表示有更改发生
                if ui.button("X").clicked() {
                    change = true;
                }
                // 如果文本编辑框失去焦点或者点击了其他地方, 退出编辑状态
                if respon.lost_focus() || respon.clicked_elsewhere() {
                    self.edit = false;
                }
            } else {
                // 如果不在编辑状态, 使用复选框显示 `active` 状态, 并显示 `title` 文本
                ui.checkbox(&mut self.active, &self.title);
                // 如果点击了编辑按钮, 进入编辑状态
                if ui.button("🖊").clicked() {
                    self.edit = true;
                }
            }
        });
        // println!("change: {}", change);
        change
    }
}
