use std::io;

use io::BufRead;

use serde_json::Value;

use cel::{Context, Program};

pub struct CelValue(pub cel::Value);

impl CelValue {
    pub fn to_bool(&self) -> Option<bool> {
        match self.0 {
            cel::Value::Bool(b) => Some(b),
            _ => None,
        }
    }
}

pub struct CelProgram(pub Program);

impl CelProgram {
    pub fn execute(&self, ctx: &Context) -> Result<CelValue, io::Error> {
        self.0.execute(ctx).map(CelValue).map_err(io::Error::other)
    }

    pub fn execute_with_value(
        &self,
        ctx: &Context,
        vname: &str,
        val: Value,
    ) -> Result<CelValue, io::Error> {
        let mut child: Context = ctx.new_inner_scope();
        child.add_variable(vname, val).map_err(io::Error::other)?;
        self.execute(&child)
    }

    pub fn filter_value(&self, ctx: &Context, vname: &str, val: &Value) -> Result<bool, io::Error> {
        let cv: CelValue = self.execute_with_value(ctx, vname, val.clone())?;
        let ob: Option<bool> = cv.to_bool();
        ob.ok_or(io::Error::other("bool expected"))
    }
}

impl CelProgram {
    pub fn jsons2filtered<I>(
        self,
        jsons: I,
        ctx: Context,
        vname: String,
    ) -> impl Iterator<Item = Result<Value, io::Error>>
    where
        I: Iterator<Item = Result<Value, io::Error>>,
    {
        jsons.filter(move |rv| match rv {
            Ok(v) => {
                let rflt = self.filter_value(&ctx, &vname, v);
                rflt.unwrap_or(false)
            }
            Err(_e) => true,
        })
    }
}

pub fn compile(expr: &str) -> Result<CelProgram, io::Error> {
    Program::compile(expr)
        .map(CelProgram)
        .map_err(io::Error::other)
}

pub fn rdr2jsons<R>(rdr: R) -> impl Iterator<Item = Result<Value, io::Error>>
where
    R: BufRead,
{
    rdr.lines()
        .map(|rline| rline.and_then(|line| serde_json::from_str(&line).map_err(io::Error::other)))
}

pub fn stdin2jsons() -> impl Iterator<Item = Result<Value, io::Error>> {
    rdr2jsons(io::stdin().lock())
}
