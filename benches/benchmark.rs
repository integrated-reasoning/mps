use criterion::*;
use mps::Parser;
cfg_if::cfg_if! {
  if #[cfg(feature = "located")] {
    use nom_locate::LocatedSpan;
    use nom_tracable::TracableInfo;
  }
}

fn netlib(c: &mut Criterion) {
  let files = [
    ("agg", include_str!("../data/netlib/agg")),
    ("ship04l", include_str!("../data/netlib/ship04l")),
    ("d2q06c", include_str!("../data/netlib/d2q06c")),
    ("e226", include_str!("../data/netlib/e226")),
    ("nl25fv47", include_str!("../data/netlib/25fv47")),
    ("bore3d", include_str!("../data/netlib/bore3d")),
    ("ganges", include_str!("../data/netlib/ganges")),
    ("adlittle", include_str!("../data/netlib/adlittle")),
    ("forplan", include_str!("../data/netlib/forplan")),
    ("sc205", include_str!("../data/netlib/sc205")),
    ("nl80bau3b", include_str!("../data/netlib/80bau3b")),
    ("scrs8", include_str!("../data/netlib/scrs8")),
    ("wood1p", include_str!("../data/netlib/wood1p")),
    ("boeing1", include_str!("../data/netlib/boeing1")),
    ("kb2", include_str!("../data/netlib/kb2")),
    ("ship08s", include_str!("../data/netlib/ship08s")),
    ("scfxm1", include_str!("../data/netlib/scfxm1")),
    ("agg2", include_str!("../data/netlib/agg2")),
    ("finnis", include_str!("../data/netlib/finnis")),
    ("dfl001", include_str!("../data/netlib/dfl001")),
    ("pilot87", include_str!("../data/netlib/pilot87")),
    ("sctap1", include_str!("../data/netlib/sctap1")),
    ("agg3", include_str!("../data/netlib/agg3")),
    ("grow7", include_str!("../data/netlib/grow7")),
    ("scorpion", include_str!("../data/netlib/scorpion")),
    ("maros", include_str!("../data/netlib/maros")),
    ("shell", include_str!("../data/netlib/shell")),
    ("greenbeb", include_str!("../data/netlib/greenbeb")),
    ("sc50b", include_str!("../data/netlib/sc50b")),
    ("recipe", include_str!("../data/netlib/recipe")),
    ("sierra", include_str!("../data/netlib/sierra")),
    ("scagr25", include_str!("../data/netlib/scagr25")),
    ("modszk1", include_str!("../data/netlib/modszk1")),
    ("ship12l", include_str!("../data/netlib/ship12l")),
    ("stair", include_str!("../data/netlib/stair")),
    ("cycle", include_str!("../data/netlib/cycle")),
    ("sc105", include_str!("../data/netlib/sc105")),
    ("pilot_ja", include_str!("../data/netlib/pilot.ja")),
    ("beaconfd", include_str!("../data/netlib/beaconfd")),
    ("czprob", include_str!("../data/netlib/czprob")),
    ("pilot_we", include_str!("../data/netlib/pilot.we")),
    ("standgub", include_str!("../data/netlib/standgub")),
    ("standmps", include_str!("../data/netlib/standmps")),
    ("scsd8", include_str!("../data/netlib/scsd8")),
    ("woodw", include_str!("../data/netlib/woodw")),
    ("scsd6", include_str!("../data/netlib/scsd6")),
    ("scsd1", include_str!("../data/netlib/scsd1")),
    ("share2b", include_str!("../data/netlib/share2b")),
    ("gfrd_pnc", include_str!("../data/netlib/gfrd-pnc")),
    ("bnl2", include_str!("../data/netlib/bnl2")),
    ("stocfor2", include_str!("../data/netlib/stocfor2")),
    ("nesm", include_str!("../data/netlib/nesm")),
    ("share1b", include_str!("../data/netlib/share1b")),
    ("ship04s", include_str!("../data/netlib/ship04s")),
    ("grow15", include_str!("../data/netlib/grow15")),
    ("maros_r7", include_str!("../data/netlib/maros-r7")),
    ("blend", include_str!("../data/netlib/blend")),
    ("lotfi", include_str!("../data/netlib/lotfi")),
    ("standata", include_str!("../data/netlib/standata")),
    ("d6cube", include_str!("../data/netlib/d6cube")),
    ("degen3", include_str!("../data/netlib/degen3")),
    ("capri", include_str!("../data/netlib/capri")),
    ("grow22", include_str!("../data/netlib/grow22")),
    ("etamacro", include_str!("../data/netlib/etamacro")),
    ("ship08l", include_str!("../data/netlib/ship08l")),
    ("afiro", include_str!("../data/netlib/afiro")),
    ("degen2", include_str!("../data/netlib/degen2")),
    ("boeing2", include_str!("../data/netlib/boeing2")),
    ("fit1d", include_str!("../data/netlib/fit1d")),
    ("scfxm2", include_str!("../data/netlib/scfxm2")),
    ("sctap3", include_str!("../data/netlib/sctap3")),
    ("fit1p", include_str!("../data/netlib/fit1p")),
    ("pilot", include_str!("../data/netlib/pilot")),
    ("fit2d", include_str!("../data/netlib/fit2d")),
    ("sctap2", include_str!("../data/netlib/sctap2")),
    ("scfxm3", include_str!("../data/netlib/scfxm3")),
    ("brandy", include_str!("../data/netlib/brandy")),
    ("greenbea", include_str!("../data/netlib/greenbea")),
    ("tuff", include_str!("../data/netlib/tuff")),
    ("sc50a", include_str!("../data/netlib/sc50a")),
    ("vtp_base", include_str!("../data/netlib/vtp.base")),
    ("pilotnov", include_str!("../data/netlib/pilotnov")),
    ("ship12s", include_str!("../data/netlib/ship12s")),
    ("seba", include_str!("../data/netlib/seba")),
    ("fffff800", include_str!("../data/netlib/fffff800")),
    ("israel", include_str!("../data/netlib/israel")),
    ("perold", include_str!("../data/netlib/perold")),
    ("pilot4", include_str!("../data/netlib/pilot4")),
    ("scagr7", include_str!("../data/netlib/scagr7")),
    ("bandm", include_str!("../data/netlib/bandm")),
    ("bnl1", include_str!("../data/netlib/bnl1")),
    ("stocfor1", include_str!("../data/netlib/stocfor1")),
  ];

  let mut group = c.benchmark_group("netlib");
  for (name, content) in files.iter() {
    group.throughput(Throughput::Bytes(content.len() as u64));
    let bench_name = format!("Parser::parse({})", name);
    group.bench_with_input(
      BenchmarkId::from_parameter(bench_name),
      content,
      |b, &content| {
        b.iter(|| {
          cfg_if::cfg_if! {
            if #[cfg(feature = "located")] {
              let info = TracableInfo::new().forward(false).backward(false);
              Parser::<f32>::parse(LocatedSpan::new_extra(content, info))
            } else {
              Parser::<f32>::parse(&content)
            }
          }
        });
      },
    );
  }
  group.finish();
}

criterion_group!(benches, netlib);
criterion_main!(benches);
