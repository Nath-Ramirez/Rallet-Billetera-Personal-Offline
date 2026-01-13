mod wallet;
mod expense;
mod storage;

use chrono::Local;
use crate::wallet::Wallet;
use crate::expense::Expense;


fn main() {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let month = Local::now().format("%Y-%m").to_string();

    let mut wallet = storage::load_wallet().unwrap_or_else(|| {
        Wallet::new(month.clone(), today.clone())
    });

    wallet.check_day_change(&today);

    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("add") => {
            let amount: i64 = args
                .get(2)
                .expect("Debes indicar el monto")
                .parse()
                .expect("El monto debe ser un número");

            let category = args
                .get(3)
                .expect("Debes indicar una categoría")
                .to_string();

            let note = args.get(4).map(|s| s.to_string());

            let expense = Expense {
                amount,
                category,
                date: today.clone(),
                note,
            };

            wallet.daily_expenses.push(expense);

            println!("Gasto agregado correctamente");

        }
        Some("today") => {
            let total: i64 = wallet
                .daily_expenses
                .iter()
                .map(|e| e.amount)  //Accedemos al amount de cada expenses guardado en el vector daily_expenses
                .sum();             //Sumamos todos los amount para obtener el valor total de los gastos del día

            println!("Fecha: {}", wallet.current_day);
            println!("Gasto total del día: {}", total);

        }
        Some("daily-list") => {
            println!("Gastos del día: {}", wallet.current_day);
            println!("--------------------------------");

            let mut total: i64 = 0;

            for (index, expense) in wallet.daily_expenses.iter().enumerate() {
                total += expense.amount;

                print!(
                    "{}. {:<12} {:>8}",
                    index + 1,
                    expense.category,
                    expense.amount
                );

                if let Some(note) = &expense.note {
                    print!("  {}", note);
                }

                println!();
            }

            println!("--------------------------------");
            println!("Total: {}", total);

        }
        _ => {
            println!("Comando no reconocido");
        }
    }

    storage::save_wallet(&wallet).expect("Error guardando datos");
}
