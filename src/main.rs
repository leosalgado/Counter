use druid::{Data, widget::{Label, Flex,  Button}, Env, Widget, WindowDesc, AppLauncher};
/*
 * Data
 * UI Builder
 * Main
 */

#[derive(Clone, Data)]
struct LeoData{
    num: i32
}

fn ui_builder() -> impl Widget<LeoData>{
    // Counter: _
    // + -
    let label = Label::new(|data: &LeoData, _: &Env| format!("Counter: {}", data.num));
    let increment = Button::new("+")
        .on_click(|_ctx, data: &mut LeoData, _env | data.num += 1);
    let decrement= Button::new("-")
        .on_click(|_ctx, data: &mut LeoData, _env | data.num -= 1);

    Flex::column().with_child(label).with_child(Flex::row().with_child(increment).with_child(decrement))
}

fn main() {
    // Window Descriptor
    // Launch to the stars
    let main_window = WindowDesc::new(ui_builder())
        .title("Leo");
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(LeoData { num: 0 }).unwrap();
}
