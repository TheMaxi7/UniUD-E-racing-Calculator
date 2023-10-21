use shunting::ShuntingParser;
use shunting::MathContext;
use slint::SharedString;
slint::slint! {
    import { GridBox, VerticalBox, Switch} from "std-widgets.slint";
        
    export global Logic {
        callback btn-clicked(string);
    }
         
    export component Button {
        callback clicked;
        callback toggled;
        in property<string> btn_text;
        in property<string> btn_value : btn-text;
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
                    Logic.btn-clicked(root.btn_value);
                }
            }
        }
        Text { 
            text: root.btn-text;
            color: root.color;
            font-size: 19px;
        }
    }    
    export component App inherits Window {
        
        in property <string> value: "0";
        title: "UniUd E-Racing Team Calculator";
        icon: @image-url("C:\\Users\\david\\rust\\calculator\\media\\logo.png");
        min-width: 360px;
        min-height: 470px;
        VerticalBox { 
           /*Switch {
                width: 10px;
                height: 10px;
                text: "Race mode";
                toggled => {
                }
            }*/
            Text {
                font-size: 65px;
                text: value;
                vertical-alignment: center;
                horizontal-alignment: right;
            }  
            GridBox {
                
                Row {
                    Button { 
                        btn_text: "%";  
                    }
                    Button { 
                        btn_text: "Ans";
                        btn_value: "";
                    }
                    Button { 
                        btn_text: "C"; 
                    }
                    Button { 
                        btn_text: "<"; 
                    }
                }
                Row {
                    Button { 
                        btn_text: "x³";
                        btn_value: "^3"; 
                    }
                    Button { 
                        btn_text: "x²";
                        btn_value: "^2"; 
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
                        btn_value: "*"; 
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
                        btn_value: "(-)"; 
                    }
                    Button { 
                        btn_text: "0"; 
                    }
                    Button { 
                        btn_text: "."; 
                    }
                    Button { 
                        btn_text: "="; 
                        background:cyan; 
                        color:black; 
                    }
                }
            }   
        }            
    }
}

fn main() {
    let mut sign = false;
    let mut previous_result = String::new();
    let mut current_value = String::new();
    let calc = App::new().unwrap();
    let weak_calc = calc.as_weak();
    let logic = calc.global::<Logic>();
    logic.on_btn_clicked(move |value| {
        let calc = weak_calc.unwrap();
        current_value += &value;
        calc.set_value(current_value.clone().into());                
        if value == "="{
            current_value.pop();
            //todo
            if current_value.contains('√') && !current_value.ends_with('√') && !current_value.starts_with('√'){
                current_value = current_value.replace("√", "*") + "^0.5";
            } else if current_value.contains('√') && current_value.starts_with('√'){
                current_value = current_value.replace("√", "") + "^0.5";
            } else if current_value.contains('√') && current_value.ends_with('√'){
                calc.set_value("Sintax error".into());
                current_value.clear();
            }

            if current_value.contains('%') && !current_value.ends_with('%') {
                current_value = current_value.replace("%", "/100*");
            } else {
                current_value = current_value.replace("%", "/100");
            }
            let current_value_as_str = current_value.as_str();
            let expr = ShuntingParser::parse_str(current_value_as_str).unwrap();
            if let Ok(result) = MathContext::new().eval(&expr){
                current_value.clear();
                previous_result.clear();
                calc.set_value(SharedString::from(result.to_string()));  
                current_value += &result.to_string();
                previous_result += &result.to_string();
            } else {
                calc.set_value("Sintax error".into());
                current_value.clear();
            }   
        } else if value == "" {
            current_value+= &previous_result;
            calc.set_value(SharedString::from(current_value.to_string()));
            previous_result.clear();
        } else if value == "C" {
            current_value.clear();
            current_value += "0";   
            calc.set_value(SharedString::from(current_value.to_string()));
            current_value.clear();
        //todo       
        } else if value == "(-)" {
            if sign == true {
                current_value.pop();
                sign = false;
                calc.set_value(SharedString::from(current_value.to_string()));
            } else{
                calc.set_value(SharedString::from(current_value.to_string()));
                sign = true;
            }
        } else if value == "<" {
            current_value.pop();
            current_value.pop();
            calc.set_value(SharedString::from(current_value.to_string()));
            if current_value == ""{ 
                current_value += "0";
                calc.set_value(SharedString::from(current_value.to_string()));
                current_value.clear();  
            } 
                
        } 
    });
        calc.run().unwrap();
    }





    /*
    
    
    fn main() {
    let mut prev_value = 0.0;
    let mut last_value = 0.0;
    let mut result = 0.0;
    let mut current_value =0.0;
    let calc = App::new().unwrap();
    let weak_calc = calc.as_weak();
    let logic = calc.global::<Logic>();
    logic.on_btn_clicked(move |value| {
        let calc = weak_calc.unwrap();
        if let Ok(parsed_value) = value.parse::<f32>() {
            current_value = calc.get_value();
            calc.set_value(current_value * 10.0 + parsed_value);
            last_value=current_value * 10.0 + parsed_value;                
        }
        if value.as_str() == "+" {
            prev_value = last_value;
            result = prev_value + last_value;
            last_value = result;    
        } else if value.as_str() == "-" {
            prev_value = last_value;
            result = prev_value - last_value;
            last_value = result;
        } else if value.as_str() == "x" {
            prev_value = last_value;
            result = prev_value * last_value;
            last_value = result;
        } else if value.as_str() == "/" {
            prev_value = last_value;
            result = prev_value / last_value;
            last_value = result;
        } else if value.as_str() == "=" {
            calc.set_value(result);
            last_value = 0.0;
            prev_value = result;
        } else if value.as_str() == "%" {
            calc.set_value(result);
            last_value = 0.0;
            prev_value = result;
        } else if value.as_str() == "CE" {
            calc.set_value(0.0);
            last_value = 0.0;
            prev_value = result;
        } else if value.as_str() == "C" {
            calc.set_value(0.0);
            last_value = 0.0;
            prev_value = 0.0;
        } else if value.as_str() == "x³" {
            prev_value = last_value;
            result = prev_value * prev_value * prev_value;
            last_value = result;
            calc.set_value(result);
        } else if value.as_str() == "x²" {
            prev_value = last_value;
            result = prev_value * prev_value;
            last_value = result;
            calc.set_value(result);
        } else if value.as_str() == "√" {
            prev_value = last_value;
            result = prev_value.sqrt();
            last_value = result;
            calc.set_value(result);
        } else if value.as_str() == "<<" {
            calc.set_value(11111111.0);
        }    
    });
        calc.run().unwrap();
    }

 */