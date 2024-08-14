//use std::result;

slint::include_modules!();

const TAXPER: f64 = 0.30;
const OWNERPER: f64 = 0.55;
const PROFPER: f64 = 0.05;
const OPERPER: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_divide_income({
        let ui_handle = ui.as_weak();
        move |string| {
            let ui = ui_handle.unwrap();
            if let Ok(num) = string.trim().parse::<f64>() {
                let tax = num * TAXPER;
                let owner = num * OWNERPER;
                let profit = num * PROFPER;
                let operation = num * OPERPER;
                let result = format!(
                    "Taxes: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOperEx: {:.2}",
                    tax, owner, profit, operation
                );

                ui.set_result(result.into());
            } else {
                // Handle the case where parsing fails
                ui.set_result("Invalid input".to_string().into());
            }
        }
    });

    ui.run()
}