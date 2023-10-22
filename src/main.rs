use shunting::ShuntingParser;
use shunting::MathContext;
use slint::SharedString;
slint::include_modules!();

fn main() {
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
        } else if value == "AC" {
            current_value.clear();
            previous_result.clear();   
            calc.set_value(SharedString::from(current_value.to_string()));
        } else if value == "<" {
            current_value.pop();
            current_value.pop();
            calc.set_value(SharedString::from(current_value.to_string()));
            if current_value == "" { 
                current_value += "0";
                calc.set_value(SharedString::from(current_value.to_string()));
                current_value.clear();  
            }          
        } 
    });
    calc.run().unwrap();
}
