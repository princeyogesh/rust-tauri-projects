
use leptos::{leptos_dom::logging::console_log, prelude::*, prelude::NodeRef};
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
   let new_expense_name = RwSignal::new(String::new());
   let new_expense_amount = RwSignal::new(String::new());
   let new_expense_category = RwSignal::new(String::new());
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
            <DurationView />
            <Button on_click=move |_| open.set(true)>"Enter new Expense"</Button>
            <CDialog curstate = open exp_name=new_expense_name exp_amount=new_expense_amount exp_category= new_expense_category />
            <h1>"Total Expenses: " {exp_sig} </h1>
        </ConfigProvider>    
    }
}

#[component]
pub fn CDialog(curstate: RwSignal<bool>, exp_name:RwSignal<String>, exp_amount: RwSignal<String>, exp_category: RwSignal<String>) -> impl IntoView {
    let setter = use_context::<WriteSignal<i32>>().expect("Setter not found in context");
    view! {
        <Dialog open =curstate>
        <DialogSurface>
            <DialogBody>
                <DialogTitle>"Creating new expense"</DialogTitle>
                <DialogContent>
                    <TextField label="Expense Name".to_string() placeholder="Enter the name of the expense".to_string() RWsig= exp_name />
                    <TextField label="Amount".to_string() placeholder="Enter the amount".to_string() RWsig=exp_amount />
                    <CDatePicker  />
                   
                    <TextField label="Category".to_string() placeholder="Enter the category of the expense".to_string() RWsig=exp_category/>
                </DialogContent>
                <DialogActions>
                    <Button on_click=move|_| {*setter.write() += exp_amount.get().parse::<i32>().unwrap_or(3) ;} appearance=ButtonAppearance::Primary >"Confirm the expense"</Button>
                </DialogActions>
            </DialogBody>
        </DialogSurface>
    </Dialog>
    }
}

#[component]
pub fn TextField(label :String, placeholder: String, RWsig:RwSignal<String>)-> impl IntoView {

    view! {
        <Layout has_sider=true>
            <LayoutSider>            
                <label for=label>{{placeholder.clone()}}</label>
            </LayoutSider>
                <Textarea />
        </Layout>
       
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
#[component]
pub fn DurationView() -> impl IntoView {
    let open = RwSignal::new(false);
    let position = RwSignal::new(DrawerPosition::Top);
    let viewtype = RwSignal::new(String::new());

    let open_f = move |new_position: DrawerPosition, v_type: String| {
        position.set(new_position);
        viewtype.set(v_type);
        open.set(true);
    };
    view! {
        <Button on_click=move |_| open_f(DrawerPosition::Left, "monthly".to_string())>"MonthlyView"</Button>
        <Button on_click=move |_| open_f(DrawerPosition::Left, "yearly".to_string())>"AnnualView"</Button>

        <OverlayDrawer open position>
        <DrawerHeader>
          <DrawerHeaderTitle>
            <DrawerHeaderTitleAction slot>
                 <Button
                    appearance=ButtonAppearance::Subtle
                    on_click=move |_| open.set(false)
                >
                    "x"
                </Button>
            </DrawerHeaderTitleAction>
             {viewtype} " View"
          </DrawerHeaderTitle>
        </DrawerHeader>
        <DrawerBody>
          <TablePanel v_type = viewtype.get() />
        </DrawerBody>
    </OverlayDrawer>
    }
}

#[component]
pub fn TablePanel(v_type: String) -> impl IntoView {    //v_type is used to determine the type of view (monthly or yearly) and can be used to fetch data accordingly
    view! {
    <Table>
        <TableHeader>
            <TableRow>
                <TableHeaderCell>"Date"</TableHeaderCell>
                <TableHeaderCell>"Description"</TableHeaderCell>
                <TableHeaderCell>"Cost"</TableHeaderCell>
            </TableRow>
        </TableHeader>
        
    </Table>
    }
}
                