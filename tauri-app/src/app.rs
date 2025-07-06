
use leptos::{leptos_dom::logging::console_log, prelude::*};
use leptos::task::spawn_local;
use leptos::*;
use serde::Serialize;
use thaw::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use chrono::prelude::*;
use serde::{Deserialize};
use wasm_bindgen::JsValue;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"] )]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct InitExpenseArgs<'a> {
    dummyarg: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    
   console_log("HELLO FROM LEPTOS");
   let open = RwSignal::new(false);
   let (exp_sig, set_exp_sig) = signal(0);
   let get_till_now_expenses = move || {
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&InitExpenseArgs { dummyarg: "yogsh" }).unwrap();
            let resp = invoke("inital_expense", args).await.as_f64().unwrap();
            set_exp_sig.set(resp as i32);
        });
    };
    get_till_now_expenses();  //This will fetch the total expenses when the app starts
    // We also provide the expense signal itself so that it can be used in the view
    

   provide_context(set_exp_sig);

    // This is the main entry point of the application
    // It renders a button that opens a dialog to enter a new expense
    // The dialog contains fields for expense name, amount, date, and category
    // The dialog can be opened by clicking the button  
   view! {
        <ConfigProvider>
             <Button on_click=move |_| open.set(true)>"Enter new Expense"</Button>
            <CDialog curstate = open/>
            <h1>"Total Expenses: " {exp_sig} </h1>
        </ConfigProvider>    
    }
}

#[component]
pub fn CDialog(curstate: RwSignal<bool>) -> impl IntoView {
    let setter = use_context::<WriteSignal<i32>>().expect("Setter not found in context");
    view! {
        <Dialog open =curstate>
        <DialogSurface>
            <DialogBody>
                <DialogTitle>"Creating new expense"</DialogTitle>
                <DialogContent>
                    <TextField label="Expense Name".to_string() placeholder="Enter the name of the expense".to_string() />
                    <TextField label="Amount".to_string() placeholder="Enter the amount".to_string() />
                    <CDatePicker  />
                    <TextField label="Category".to_string() placeholder="Enter the category of the expense".to_string()/>
                </DialogContent>
                <DialogActions>
                    <Button on_click=move|_| {*setter.write() += 10;} appearance=ButtonAppearance::Primary >"Confirm the expense"</Button>
                </DialogActions>
            </DialogBody>
        </DialogSurface>
    </Dialog>
    }
}

#[component]
pub fn TextField(label :String, placeholder: String)-> impl IntoView {

    view! {
        <Layout has_sider=true>
            <LayoutSider>            
                <label for=label>{{placeholder.clone()}}</label>
            </LayoutSider>
                <Textarea />
        </Layout>
           // <input type="text" id={{label}} class="form-control" placeholder="Enter the name of the expense"/>
       
    }
}
#[component]
pub fn CDatePicker() -> impl IntoView {
    let date_now = RwSignal::new(Local::now().date_naive());

    view! {
        <Layout has_sider=true>
            <LayoutSider>
                <label for="date">"Date"</label>
            </LayoutSider>
            <DatePicker
                value = date_now />
        </Layout>
    }
}   
                