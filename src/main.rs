use druid::{Data, widget::{Label,Button,Flex}, Env, Widget,WindowDesc,AppLauncher};
/**
Data
UI Builder
Main
*/

#[derive(Clone,Data)]
struct FunnyData{
    num:i32,
}

fn ui_builder() -> impl Widget<FunnyData>{
    let label = Label::new(| data:&FunnyData , _:&Env | format!("Counter {}",data.num));
    let increment = Button::new("+")
                            .on_click(|_ctx, data : &mut FunnyData, _env| data.num+=1);
    let decrement = Button::new("-")
                            .on_click(|_ctx, data : &mut FunnyData, _env| data.num-=1);

    let button_row = Flex::row().with_child(increment).with_child(decrement);

    Flex::column().with_child(label).with_child(button_row)
}


fn main() {
    // Window Discriptor 
    // Launch the program
    let main_window = WindowDesc::new(ui_builder()).title("Funny");
    AppLauncher::with_window(main_window).log_to_console().launch(FunnyData {num:0}).unwrap();
}
