use anyhow::{Error, Result};
use shared::LogLine;
use std::cell::Cell;
use std::env::args;
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use wasmer_runtime::{error, imports, instantiate, Array, Func, Instance, Memory, WasmPtr};

struct Plugin {
    pub instance: Instance,
}

impl<'a> Plugin {
    fn mem_ptr(&self) -> Result<WasmPtr<u8, Array>> {
        self.instance
            .exports
            .get::<Func<(), WasmPtr<u8, Array>>>("get_ptr")?
            .call()
            .map_err(|e| Error::msg(e.to_string()))
    }
    pub fn memory_cells(&self, start: u32, len: u32) -> Result<&[Cell<u8>]> {
        let mem = self.instance.context().memory(0);
        let ptr = self.mem_ptr()?;
        ptr.deref(mem, start, len)
            .ok_or_else(|| Error::msg("No memory!"))
    }

    pub fn set_log_line(&self, ll: LogLine) -> Result<()> {
        let encoded: Vec<u8> = bincode::serialize(&ll)?;
        let memory = self.memory_cells(0, encoded.len() as u32 + 2)?;
        let len = (encoded.len() as u16).to_le_bytes();
        memory[0].set(len[0]);
        memory[1].set(len[1]);
        for (i, b) in encoded.into_iter().enumerate() {
            memory[i + 2].set(b);
        }

        Ok(())
    }

    pub fn memory_string(&self, len: u32) -> Result<String> {
        let mem = self.instance.context().memory(0);
        let ptr = self.mem_ptr()?;
        ptr.get_utf8_string(mem, len)
            .map(|s| s.to_string())
            .ok_or_else(|| Error::msg("No memory!"))
    }
}

fn main() -> Result<()> {
    let a: Vec<_> = args().collect();
    if a.len() < 2 {
        return Err(Error::msg("Need a wasm file path"));
    }
    let mut wasm_bytes = vec![];
    let mut f = File::open(&a[1])?;
    let _ = File::read_to_end(&mut f, &mut wasm_bytes);
    let val = if a.len() < 3 {
        "Hello Rust WASM!".to_string()
    } else {
        a[2].clone()
    };

    let import_object = imports! {};

    println!("Instantiating WASM");
    let start = Instant::now();
    let plugin = Plugin {
        instance: instantiate(&wasm_bytes, &import_object)
            .map_err(|e| Error::msg(e.to_string()))?,
    };
    let done = Instant::now();
    println!("Instantiation took {} ms", (done - start).as_millis());

    println!("Setting memory to {}", val);

    let start = Instant::now();

    plugin.set_log_line(LogLine {
        message: val.clone(),
    })?;

    let done = Instant::now();
    println!(
        "Memory initialization took {} ms",
        (done - start).as_millis()
    );

    let test: Func<(), u8> = plugin.instance.exports.get("test_get_line")?;
    if test.call().unwrap() > 0 {
        return Err(Error::msg("Couldn't deserialize on wasm side!"));
    }

    let start = Instant::now();
    let should_filter: Func<(), u8> = plugin.instance.exports.get("should_filter")?;
    let res = should_filter.call().unwrap() > 0;
    let done = Instant::now();
    println!("Function call took {} ms", (done - start).as_millis());

    println!(
        "Should we filter {}? {}",
        val,
        if res { "yes" } else { "no" }
    );

    Ok(())
}
