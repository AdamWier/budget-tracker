use piechart::{Color, Data};

pub struct TotalInformation {
    pub label: String,
    pub total: f32,
    pub budget_amount: f32,
    pub max_to_date: f32,
    pub projected_spending: f32,
}

impl TotalInformation {
    pub fn get_chart_data(&self) -> Vec<Data> {
        if self.total <= 0.0 {
            self.get_unspent_chart()
        } else if self.max_to_date >= self.total {
            self.get_underspent_data()
        } else if self.total > self.budget_amount {
            self.get_totally_overspent()
        } else {
            self.get_overspent_data()
        }
    }
    fn get_unspent_chart(&self) -> Vec<Data> {
        let data = vec![
            Data {
                label: "Unspent".into(),
                value: self.budget_amount,
                color: Some(Color::White.into()),
                fill: '*',
            },
            Data {
                label: "Left to spend to date".into(),
                value: self.max_to_date,
                color: Some(Color::RGB(255, 165, 0).into()),
                fill: '*',
            },
            Data {
                label: "What's left for a week from now".into(),
                value: self.projected_spending + 0.1,
                color: Some(Color::Green.into()),
                fill: '*',
            },
        ];
        data.into_iter().filter(|x| x.value > 0.0).collect()
    }
    fn get_underspent_data(&self) -> Vec<Data> {
        let diff = self.max_to_date - self.total;
        let data = vec![
            Data {
                label: "Spent".into(),
                value: self.total,
                color: Some(Color::RGB(255, 165, 0).into()),
                fill: '*',
            },
            Data {
                label: "Left to spend to date".into(),
                value: diff,
                color: Some(Color::Green.into()),
                fill: '*',
            },
            Data {
                label: "What's left for a week from now".into(),
                value: self.projected_spending - diff,
                color: Some(Color::Yellow.into()),
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
    fn get_overspent_data(&self) -> Vec<Data> {
        let diff = self.total - self.max_to_date;
        let data = vec![
            Data {
                label: "Spent".into(),
                value: self.max_to_date,
                color: Some(Color::Yellow.into()),
                fill: '*',
            },
            Data {
                label: "Overspent".into(),
                value: diff,
                color: Some(Color::Red.into()),
                fill: '*',
            },
            Data {
                label: "Unspent".into(),
                value: self.budget_amount - self.total,
                color: Some(Color::White.into()),
                fill: '*',
            },
        ];
        data.into_iter().filter(|x| x.value > 0.0).collect()
    }
    fn get_totally_overspent(&self) -> Vec<Data> {
        let diff = self.total - self.budget_amount;
        let data = vec![
            Data {
                label: "Spent".into(),
                value: self.budget_amount - diff,
                color: Some(Color::Yellow.into()),
                fill: '*',
            },
            Data {
                label: "Over max".into(),
                value: diff,
                color: Some(Color::Red.into()),
                fill: '*',
            },
        ];
        data.into_iter().filter(|x| x.value > 0.0).collect()
    }
}
