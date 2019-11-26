extern crate neon;
extern crate rayon;

use neon::prelude::*;
use rayon::prelude::*;

struct FactorialTask {
    // Factorial number to sum up to
    n: usize,
}

impl Task for FactorialTask {
    type Output = u128;
    type Error = String;
    type JsEvent = JsNumber;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        let result = (1..self.n + 1)
            .into_par_iter()
            .map(|x| x as u128)
            .reduce_with(std::ops::Mul::mul);

        if let Some(value) = result {
            Ok(value)
        } else {
            Err(String::from("Something went wrong"))
        }
    }

    fn complete(
        self,
        mut cx: TaskContext,
        result: Result<Self::Output, Self::Error>,
    ) -> JsResult<Self::JsEvent> {
        Ok(cx.number(result.unwrap() as f64))
    }
}

fn factorial(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let n = cx.argument::<JsNumber>(0)?.value() as usize;

    let task = FactorialTask { n };

    let callback = cx.argument::<JsFunction>(1)?;
    task.schedule(callback);

    Ok(cx.undefined())
}

register_module!(mut cx, { cx.export_function("factorial", factorial) });
