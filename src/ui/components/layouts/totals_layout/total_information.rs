use piechart::{Color, Data};

pub struct TotalInformation {
    pub label: String,
    pub total: f32,
    pub budget_amount: f32,
    pub max_to_date: f32,
}

impl TotalInformation {
    pub fn get_underspent_data(&self) -> Vec<Data> {
        let diff = self.max_to_date - self.total;
        let data = vec![
            Data {
                label: "Spent".into(),
                value: self.total,
                color: Some(Color::Yellow.into()),
                fill: '*',
            },
            Data {
                label: "Left to spend".into(),
                value: diff,
                color: Some(Color::Green.into()),
                fill: '*',
            },
            Data {
                label: "Unspent".into(),
                value: (0 as f32).max(self.budget_amount - self.total - (diff)),
                color: Some(Color::White.into()),
                fill: '*',
            },
        ];
        data.into_iter().filter(|x| x.value > 0.0).collect()
    }
    pub fn get_overspent_data(&self) -> Vec<Data> {
        let diff = self.total - self.max_to_date;
        let data = vec![
            Data {
                label: "Overspent by".into(),
                value: self.total - self.budget_amount,
                color: Some(Color::Red.into()),
                fill: '*',
            },
            Data {
                label: "Spent".into(),
                value: self.total,
                color: Some(Color::Yellow.into()),
                fill: '*',
            },
            Data {
                label: "Unspent".into(),
                value: self.budget_amount - self.max_to_date - diff,
                color: Some(Color::White.into()),
                fill: '*',
            },
        ];
        data.into_iter().filter(|x| x.value > 0.0).collect()
    }
}
