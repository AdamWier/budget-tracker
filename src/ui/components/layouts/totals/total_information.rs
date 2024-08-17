use piechart::{Color, Data};

const FILLER: char = '*';

pub struct TotalInformation {
    pub label: String,
    pub total: f32,
    pub budget_amount: f32,
    pub max_to_date: f32,
    pub projected_spending: f32,
}

impl TotalInformation {
    fn get_orange(&self) -> Color {
        Color::RGB(255, 165, 0)
    }
    pub fn get_chart_data(&self) -> Vec<Data> {
        let data = if self.total <= 0.0 {
            self.get_unspent_chart()
        } else if self.max_to_date >= self.total {
            self.get_underspent_data()
        } else if self.total > self.budget_amount {
            self.get_totally_overspent_data()
        } else {
            self.get_overspent_data()
        };
        data.into_iter().filter(|x| x.value > 0.0).collect()
    }
    fn get_unspent_chart(&self) -> Vec<Data> {
        vec![
            Data {
                label: "Unspent".into(),
                value: self.budget_amount,
                color: Some(Color::White.into()),
                fill: FILLER,
            },
            Data {
                label: "Left to spend to date".into(),
                value: self.max_to_date,
                color: Some(self.get_orange().into()),
                fill: FILLER,
            },
            Data {
                label: "What's left for a week from now".into(),
                value: self.projected_spending + 0.1,
                color: Some(Color::Green.into()),
                fill: FILLER,
            },
        ]
    }
    fn get_underspent_data(&self) -> Vec<Data> {
        let spent = self.total;
        let underspent_by = self.max_to_date - spent;
        let left_for_next_week = self.projected_spending - self.max_to_date;
        let unspent = self.budget_amount - spent - underspent_by - left_for_next_week;
        vec![
            Data {
                label: "Spent".into(),
                value: spent,
                color: Some(self.get_orange().into()),
                fill: FILLER,
            },
            Data {
                label: "Left to spend to date".into(),
                value: underspent_by,
                color: Some(Color::Green.into()),
                fill: FILLER,
            },
            Data {
                label: "What's left for a week from now".into(),
                value: left_for_next_week,
                color: Some(Color::Yellow.into()),
                fill: FILLER,
            },
            Data {
                label: "Unspent".into(),
                value: unspent,
                color: Some(Color::White.into()),
                fill: FILLER,
            },
        ]
    }
    fn get_overspent_data(&self) -> Vec<Data> {
        let overspent = self.total - self.max_to_date;
        let unspent = self.budget_amount - self.total;
        vec![
            Data {
                label: "Max allowed to date".into(),
                value: self.max_to_date,
                color: Some(Color::Yellow.into()),
                fill: FILLER,
            },
            Data {
                label: "Overspent".into(),
                value: overspent,
                color: Some(Color::Red.into()),
                fill: FILLER,
            },
            Data {
                label: "Unspent".into(),
                value: unspent,
                color: Some(Color::White.into()),
                fill: FILLER,
            },
        ]
    }
    fn get_totally_overspent_data(&self) -> Vec<Data> {
        let over_max = self.total - self.budget_amount;
        let rest_of_chart = self.budget_amount - over_max;
        vec![
            Data {
                label: "Over max".into(),
                value: over_max,
                color: Some(Color::Red.into()),
                fill: FILLER,
            },
            Data {
                label: "Spent".into(),
                value: rest_of_chart,
                color: Some(Color::Yellow.into()),
                fill: FILLER,
            },
        ]
    }
}
