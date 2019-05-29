use std::fs;
use std::io::{Read, Write, Seek, Cursor, SeekFrom::Start};
use std::path::PathBuf;
use rlua::{Lua, UserData, UserDataMethods, Error, Value, Table};
use rand::{random, seq::IteratorRandom};
use rand_pcg::Pcg64Mcg;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "surgeon", about = "a flexible rom patching tool")]
struct Opt {
    #[structopt(short = "s", long = "seed")]
    seed: Option<u128>,
    rom: PathBuf,
    patch: PathBuf,
    output: PathBuf,
}


fn main() {
    let opt = Opt::from_args();
    let patch = fs::read(opt.patch).unwrap();
    let rom = fs::read(opt.rom).unwrap();
    let rom = Rom(Cursor::new(rom));
    let seed = opt.seed.unwrap_or_else(|| random::<u32>() as u128);
    let random = Random::new(seed);

    println!("Using seed {}", seed);

    let lua = Lua::new();
    let rom = lua.context(|ctx| {
        let globals = ctx.globals();
        
        globals.set("rom", rom).unwrap();
        globals.set("rng", random).unwrap();

        let math = globals.get::<_, rlua::Table>("math").unwrap();
        math.set("random", Value::Nil).unwrap();

        println!("Patching...");
        ctx.load(&patch)
            .set_name("patch").unwrap()
            .exec().unwrap();
        
        ctx.globals().get::<_, Rom>("rom").unwrap()
    });

    let rom = rom.0.into_inner();

    fs::write(opt.output, &rom).unwrap();

    println!("Done!");
}

pub struct Random(Pcg64Mcg);

impl Random {
    fn new(seed: u128) -> Self {
        Random(Pcg64Mcg::new(seed))
    }
}

impl UserData for Random {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("choose", |_ctx, this, table: Table| {
            let values = table.pairs::<Value, Value>().map(|kv| kv.unwrap().1);

            Ok(values.choose(&mut this.0))
        });
    }
}

#[derive(Clone)]
pub struct Rom(Cursor<Vec<u8>>);

impl UserData for Rom {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("read_byte", |_ctx, this, addr: usize| {
            let ref mut buf = [0];

            this.0.seek(Start(addr as u64)).map_err(Error::external)?;
            this.0.read_exact(buf).map_err(Error::external)?;
            
            Ok(buf[0])
        });

        methods.add_method_mut("write_byte", |_ctx, this, (addr, byte): (usize, u8)| {
            this.0.seek(Start(addr as u64)).map_err(Error::external)?;
            this.0.write_all(&[byte]).map_err(Error::external)?;
            
            Ok(())
        });
    }
}
