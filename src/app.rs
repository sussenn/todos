use serde::{Deserialize, Serialize};
use todos::todolist::{Enum, TodoItem};

#[derive(Default, Serialize, Deserialize)]
pub struct MyApp {
    addtodo: String,
    task: usize,
    radio: Enum,
    data: Vec<TodoItem>,
    // // 删除的索引
    // #[serde(skip)]
    del: i32,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        // 使用默认值创建 MyApp 的实例
        let mut data = Self::default();
        // 读取本地数据 尝试从传入的 CreationContext 中获取存储对象
        if let Some(storage) = cc.storage {
            // 尝试从存储中获取之前保存的 MyApp 数据
            // 如果获取成功, 则使用存储的数据初始化实例, 否则使用默认值
            data = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        // 字体设置
        Self::load_fonts(&cc.egui_ctx);
        // 设置删除索引为 -1, 表示没有待删除的项目
        data.del = -1;
        data
    }

    // 用于根据单选按钮的状态显示待办事项列表
    fn show_content(&mut self, ui: &mut eframe::egui::Ui) {
        match self.radio {
            Enum::All => {
                self.show_all(ui);
            }
            Enum::Active => {
                self.show_active(ui, false);
            }
            Enum::Completed => {
                self.show_active(ui, true);
            }
        }
    }

    // 显示所有任务
    fn show_all(&mut self, ui: &mut eframe::egui::Ui) {
        if self.data.len() > 0 {
            // 遍历任务列表中的每个项目
            for (idx, item) in &mut self.data.iter_mut().enumerate() {
                // 调用 sigleitem 方法显示每个项目, 并检查该项目是否待删除
                if item.sigleitem(ui) {
                    self.del = idx as i32;
                }
            }
        } else {
            // 如果列表为空, 则显示提示信息
            ui.label("暂无任务");
        }
    }
    // 根据完成状态显示任务
    fn show_active(&mut self, ui: &mut eframe::egui::Ui, active: bool) {
        // 过滤出所有与 `active` 参数匹配的任务
        let data = self
            .data
            .iter()
            .filter(|item| item.active == active)
            .collect::<Vec<_>>();
        if data.len() > 0 {
            // 遍历过滤后的任务列表
            for (idx, item) in &mut self.data.iter_mut().enumerate() {
                // 只显示那些与 `active` 参数匹配的任务
                if item.active == active {
                    if item.sigleitem(ui) {
                        self.del = idx as i32;
                    }
                }
            }
        } else {
            // 如果没有匹配的任务, 则显示提示信息
            if active {
                // 已完成的任务
                ui.label("已完成的任务为空");
            } else {
                // 待办的任务
                ui.label("待办的任务为空");
            }
        }
    }

    // 更新任务列表的状态
    fn dataupdate(&mut self) {
        // 过滤出所有未完成的任务
        let data = self
            .data
            .iter()
            .filter(|item| item.active == false)
            .collect::<Vec<_>>();
        // 更新任务数量
        self.task = data.len();

        // 如果有已删除的任务, 则从列表中移除它
        if self.del > -1 {
            println!("Deleting item at index: {}", self.del);
            self.data.remove(self.del as usize);
            self.del = -1;
        }
    }

    // 字体设置
    pub fn load_fonts(ctx: &eframe::egui::Context) {
        let mut fonts = eframe::egui::FontDefinitions::default();
        fonts.font_data.insert(
            "my_font".to_owned(),
            eframe::egui::FontData::from_static(include_bytes!("fonts/SIMFANG.TTF")),
        );
        fonts
            .families
            .entry(eframe::egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "my_font".to_owned());

        fonts
            .families
            .entry(eframe::egui::FontFamily::Monospace)
            .or_default()
            .push("my_font".to_owned());
        ctx.set_fonts(fonts);
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // 创建一个中央面板并显示 UI 布局
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            // 设置标题
            ui.heading("待办事项列表");
            // 创建一个文本编辑框, 允许用户输入新的待办事项, 提示文本为 "Add todo"
            let p =
                ui.add(eframe::egui::TextEdit::singleline(&mut self.addtodo).hint_text("添加任务"));
            // 如果文本编辑框失去焦点并且用户没有点击其他地方, 则处理添加待办事项
            if p.lost_focus() & !p.clicked_elsewhere() {
                // 如果输入框中有文本, 则创建一个新的 TodoItem 实例并添加到 data 列表中
                if self.addtodo.len() > 0 {
                    self.data.push(TodoItem {
                        title: self.addtodo.clone(),
                        active: false,
                        edit: false,
                    });
                    // 清空输入框
                    self.addtodo.clear();
                }
            }
            // 创建一个水平布局
            ui.horizontal(|ui| {
                // 任务数量
                ui.label(&self.task.to_string());
                // 提示文本
                ui.label("项任务待办");
                // 分隔
                ui.add_space(44.0);
                // 单选按钮
                ui.selectable_value(&mut self.radio, Enum::All, "所有");
                ui.selectable_value(&mut self.radio, Enum::Active, "待办");
                ui.selectable_value(&mut self.radio, Enum::Completed, "完结");
            });
            // 分隔
            ui.add_space(11.0);
            // 列表显示
            self.show_content(ui);
        });
        // 更新数据
        self.dataupdate();
    }

    // App关闭时,默认调用的方法  将数据保存到本地
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // 将当前的 MyApp 实例保存到 eframe 提供的存储中
        eframe::set_value(_storage, eframe::APP_KEY, self);
    }
}
