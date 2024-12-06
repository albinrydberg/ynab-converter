pub mod swedish_float;

#[derive(Default)]
pub struct Flow {
    pub inflow: f32,
    pub outflow: f32,
}

pub fn convert_amount_to_flow(amount: f32) -> Flow {
    if amount < 0.0 {
        Flow {
            outflow: amount.abs(),
            ..Flow::default()
        }
    } else {
        Flow {
            inflow: amount,
            ..Flow::default()
        }
    }
}
