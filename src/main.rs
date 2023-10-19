
slint::slint! {
import { GridBox, VerticalBox} from "std-widgets.slint";
    
export global Logic {
    callback btn-clicked(string);
}
    
    
export component Button {
    callback clicked;
    in property<string> btn_text;
    in property <brush> background: grey;
    in property <brush> color: white;
    Rectangle {
        min-width: 75px;
        min-height: 50px;
        background: area.pressed ? root.background.darker(60%) : area.has-hover ? root.background.darker(20%) : root.background;
        border-radius: 5px;
        border-width: 5px;
        border-color: transparent;
        area := TouchArea {
            clicked => {
                Logic.btn-clicked(root.btn_text);
            }
        }
    }
    Text { 
        text: root.btn_text;
        color: root.color;
        font-size: 19px;
    }
}    
export component App inherits Window {
    in property <float> value: 0;
    title: "UniUd E-Racing Team Calculator";
    icon: @image-url("C:\\Users\\david\\rust\\calculator\\media\\logo.png");
    min-width: 360px;
    min-height: 470px;
    VerticalBox { 
        Text {   
            font-size: 85px;
            text: value; 
            vertical-alignment: center;
            horizontal-alignment: right;
            wrap:word-wrap;   
        }  
        GridBox {
            Row {
                Button { 
                    btn_text: "%"; 
                }
                Button { 
                    btn_text: "CE";
                }
                Button { 
                    btn_text: "C"; 
                }
                Button { 
                    btn_text: "<<"; 
                }
            }
            Row {
                Button { 
                    btn_text: "x³"; 
                }
                Button { 
                    btn_text: "x²"; 
                }
                Button { 
                    btn_text: "√"; 
                }
                Button { 
                    btn_text: "/"; 
                }
            }
            Row {
                Button { 
                    btn_text: "7"; 
                }
                Button { 
                    btn_text: "8"; 
                }
                Button { 
                    btn_text: "9"; 
                }
                Button { 
                    btn_text: "x"; 
                }
            }
            Row {
                Button { 
                    btn_text: "4"; 
                }
                Button { 
                    btn_text: "5"; 
                }
                Button { 
                    btn_text: "6"; 
                }
                Button { 
                    btn_text: "-"; 
                }
            }
            Row {
                Button { 
                    btn_text: "1"; 
                }
                Button { 
                    btn_text: "2"; 
                }
                Button { 
                    btn_text: "3"; 
                }
                Button { 
                    btn_text: "+"; 
                }
            }
            Row {
                Button { 
                    btn_text: "+/-"; 
                }
                Button { 
                    btn_text: "0"; 
                }
                Button { 
                    btn_text: "."; 
                }
                Button { 
                    btn_text: "="; 
                    background:pink; 
                    color:black; 
                }
            }
        }   
    }            
}


}
fn main() {
    let mut last_value = 0.0;
    let mut result = 0.0;
    let calc = App::new().unwrap();
    let weak_calc = calc.as_weak();
    calc.global::<Logic>().on_btn_clicked(move |value| {
        let calc = weak_calc.unwrap();
        if is_number(&value, &last_value) {
            let Ok(parsed_value) = value.parse::<f32>() else { todo!() };
            let current_value = calc.get_value();
            calc.set_value(current_value * 10.0 + parsed_value);
            last_value=current_value * 10.0 + parsed_value;
        }
        else if is_operator(&value, &last_value) {
            calc.set_value(result);
            result += last_value;
        }
        else if &value == "="{
            calc.set_value(result);
        }
    });
    calc.run().unwrap();
}

fn is_operator(value: &str, last_value: &f32)-> bool{
    let operators = ["+","-","x","/","%"];
    operators.contains(&value)
}

fn is_number(value: &str , last_value: &f32)-> bool{
    let numbers = ["1","2","3","4","5","6","7","8","9","0"];
    numbers.contains(&value)
}
