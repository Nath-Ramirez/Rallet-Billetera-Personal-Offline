use serde::{Serialize, Deserialize};
use crate::expense::Expense;

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub monthly_income: i64,
    pub current_month: String,
    pub current_day: String,
    pub daily_expenses: Vec<Expense>,
    pub monthly_total_spent: i64,
}


impl Wallet {
    pub fn new(current_month: String, current_day: String) -> Self {
        Self {
            monthly_income: 0,
            current_month,
            current_day,
            daily_expenses: Vec::new(),
            monthly_total_spent: 0,
        }
    }
}


impl Wallet {
    pub fn check_day_change(&mut self, today: &str) {
        if self.current_day != today {
            let daily_total: i64 = self
                .daily_expenses
                .iter()             //Itera en cada uno de los gastos guardados en el vector "daily_expenses" sin usar for
                .map(|e| e.amount)  //Accedemos a cada monto de cada gasto, "e" es el iterador
                .sum();             //Suma todos los montos 

            self.monthly_total_spent += daily_total;
            self.daily_expenses.clear();  //Reiniciamos el vector de gastos diarios para iniciar un nuevo día
            self.current_day = today.to_string();
        }
    }
}
