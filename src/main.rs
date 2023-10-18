fn main() {
    MainWindow::new().unwrap().run().unwrap();

}

slint::slint! {
    import { GridBox, VerticalBox} from "std-widgets.slint";
    component Button {
        in property <string> btn_text;
        in property <brush> background: grey;
        in property <brush> color: white;
        in-out property <string> expression: "ciao";
        Rectangle {
            min-width: 75px;
            min-height: 50px;
            background: area.pressed ? root.background.darker(60%) : area.has-hover ? root.background.darker(20%) : root.background;
            border-radius: 5px;
            border-width: 5px;
            border-color: transparent;
            area := TouchArea {
                /*clicked => {
                    root.expression += root.btn_text
                }*/
            }
        }
        Text { 
            text: root.btn_text;
            color: root.color;
            font-size: 19px;
        }
    }    
    export component MainWindow inherits Window {
        title: "UniUd E-Racing Team Calculator";
        icon: @image-url("C:\\Users\\david\\rust\\calculator\\media\\logo.png");
        min-width: 360px;
        min-height: 470px;
        VerticalBox { 
            Text {   
                font-size: 85px;
                text: 0; 
                vertical-alignment: center;
                horizontal-alignment: right;
                wrap:word-wrap;   
            }
            
            GridBox {

                Row {
                    Button { btn_text: "%"; }
                    Button { btn_text: "CE";}
                    Button { btn_text: "C"; }
                    Button { btn_text: "<<"; }
                }
                Row {
                    Button { btn_text: "x³"; }
                    Button { btn_text: "x²"; }
                    Button { btn_text: "√"; }
                    Button { btn_text: "/"; }
                }
                Row {
                    Button { btn_text: "7"; }
                    Button { btn_text: "8"; }
                    Button { btn_text: "9"; }
                    Button { btn_text: "x"; }
                }
                Row {
                    Button { btn_text: "4"; }
                    Button { btn_text: "5"; }
                    Button { btn_text: "6"; }
                    Button { btn_text: "-"; }
                }
                Row {
                    Button { btn_text: "1"; }
                    Button { btn_text: "2"; }
                    Button { btn_text: "3"; }
                    Button { btn_text: "+"; }
                }
                Row {
                    Button { btn_text: "+/-"; }
                    Button { btn_text: "0"; }
                    Button { btn_text: "."; }
                    Button { btn_text: "="; background:pink; color:black;}
                }
            }   
        }        
    }     
}

