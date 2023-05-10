use druid::{AppLauncher, Widget, WindowDesc, PlatformError, WidgetExt, Data, Lens, TimerToken, EventCtx, Event, Env, BoxConstraints, LayoutCtx, LifeCycleCtx, LifeCycle, PaintCtx, RenderContext, Size};
use druid::widget::Label;
use std::io;
use std::time::Duration;
use std::sync::Arc;
use tokio::time;
use tokio::sync::Mutex;
use std::thread;
use std::ops::Deref;

#[derive(Clone, Data, Lens)]
struct AppData {
    income_per_minute: f64,
    total_income: Arc<Mutex<f64>>,
}

struct IncomeDisplay;

impl Widget<AppData> for IncomeDisplay {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppData, _env: &Env) {
        match event {
            Event::WindowConnected => {
                ctx.request_timer(Duration::from_secs(60));
            }
            Event::Timer(id) => {
                if id == &TimerToken::INVALID {
                    ctx.request_timer(Duration::from_secs(60));
                    let income_per_minute = data.income_per_minute;
                    let total_income = Arc::clone(&data.total_income);
                    thread::spawn(move || {
                        let mut income = total_income.lock().unwrap();
                        *income += income_per_minute;
                    });
                }
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &AppData, _env: &Env) {}

    fn update(&mut self, _ctx: &mut EventCtx, _old_data: &AppData, _data: &AppData, _env: &Env) {}

    fn layout(&mut self, _ctx: &mut LayoutCtx, _bc: &BoxConstraints, _data: &AppData, _env: &Env) -> Size {
        Size::new(100.0, 100.0)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, _env: &Env) {
        let income = data.total_income.lock().unwrap();
        let text = format!("現在の総収入: {:.2} 円", income.deref());
        let label = Label::new(text);
        label.paint(ctx, data, _env);
    }
}

fn main() -> Result<(), PlatformError> {
    println!("年収を入力してください（円）: ");

    let mut annual_income = String::new();
    io::stdin().read_line(&mut annual_income)
        .expect("Failed to read line");

    let annual_income: f64 = match annual_income.trim().parse()
    {
        Ok(num) => num,
        Err(_) => return,
    };

    let working_days_per_year = 120.0;
    let working_hours_per_day = 8.0;
    let minutes_per_hour = 60.0;

    // Calculate income per minute
    let income_per_minute = annual_income / (working_days_per_year * working_hours_per_day * minutes_per_hour);
    let total_income = Arc::new(Mutex::new(0.0));
    let data = AppData {
        income_per_minute,
        total_income: Arc::clone(&total_income),
    };

    let main_window = WindowDesc::new(IncomeDisplay).title("Income Tracker");
    AppLauncher::with_window(main_window)
        .launch(data)
}
