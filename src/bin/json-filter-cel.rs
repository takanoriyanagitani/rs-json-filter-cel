use std::io;

use io::BufWriter;
use io::Write;

use serde_json::Value;

use cel::Context;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The CEL expression to filter JSON objects
    expr: String,

    /// The name of the variable for the JSON object in the CEL expression
    #[arg(long, default_value = "item")]
    json_obj_name: String,
}

fn sub(expr: &str, ctx: Context, vname: String) -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();
    let mut bw = BufWriter::new(&mut ol);

    let prog = rs_json_filter_cel::compile(expr)?;
    let jsons = rs_json_filter_cel::stdin2jsons();
    let filtered = prog.jsons2filtered(jsons, ctx, vname);
    for rval in filtered {
        let val: Value = rval?;
        serde_json::to_writer(&mut bw, &val)?;
        writeln!(&mut bw)?;
    }

    bw.flush()?;
    drop(bw);
    ol.flush()?;
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let cli = Cli::parse();
    sub(&cli.expr, Context::default(), cli.json_obj_name)
}
