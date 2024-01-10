mod tests {
  use color_eyre::Result;
  use num_traits::float::Float;
  cfg_if::cfg_if! {
    if #[cfg(feature = "trace")] {
      use nom_locate::LocatedSpan;
      use nom_tracable::TracableInfo;
    }
  }

  fn parse<T: Float>(input: &'static str) -> Result<mps::Parser<'_, f32>> {
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let info = TracableInfo::new().forward(false).backward(false);
        let (_, parsed) = mps::Parser::<T>::parse(LocatedSpan::new_extra(input, info))?;
      } else {
        let (_, parsed) = mps::Parser::<T>::parse(&input)?;
      }
    }
    Ok(parsed)
  }

  #[test]
  fn test_parse_agg() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/agg"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_ship04l() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/ship04l"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_d2q06c() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/d2q06c"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_e226() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/e226"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_nl25fv47() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/25fv47"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_bore3d() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/bore3d"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_ganges() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/ganges"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_adlittle() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/adlittle"
    ))?);
    Ok(())
  }

  #[ignore] // TODO: Fix (fails in row_line and columns)
  fn _test_parse_forplan() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/forplan"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_sc205() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/sc205"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_nl80bau3b() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/80bau3b"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_scrs8() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/scrs8"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_wood1p() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/wood1p"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_boeing1() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/boeing1"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_kb2() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/kb2"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_ship08s() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/ship08s"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_scfxm1() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/scfxm1"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_agg2() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/agg2"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_finnis() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/finnis"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_dfl001() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/dfl001"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_pilot87() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/pilot87"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_sctap1() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/sctap1"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_agg3() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/agg3"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_grow7() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/grow7"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_scorpion() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/scorpion"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_maros() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/maros"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_shell() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/shell"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_greenbeb() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/greenbeb"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_sc50b() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/sc50b"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_recipe() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/recipe"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_sierra() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/sierra"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_scagr25() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/scagr25"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_modszk1() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/modszk1"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_ship12l() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/ship12l"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_stair() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/stair"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_cycle() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/cycle"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_sc105() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/sc105"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_pilot_ja() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/pilot.ja"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_beaconfd() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/beaconfd"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_czprob() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/czprob"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_pilot_we() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/pilot.we"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_standgub() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/standgub"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_standmps() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/standmps"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_scsd8() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/scsd8"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_woodw() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/woodw"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_scsd6() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/scsd6"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_scsd1() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/scsd1"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_share2b() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/share2b"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_gfrd_pnc() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/gfrd-pnc"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_bnl2() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/bnl2"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_stocfor2() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/stocfor2"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_nesm() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/nesm"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_share1b() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/share1b"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_ship04s() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/ship04s"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_grow15() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/grow15"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_maros_r7() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/maros-r7"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_blend() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/blend"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_lotfi() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/lotfi"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_standata() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/standata"
    ))?);
    Ok(())
  }

  ////#[test]
  ////fn test_parse_d6cube() -> Result<()> {
  //  insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
  //    "../tests/data/netlib/d6cube"
  //  ))?);
  //  Ok(())
  ////}

  #[test]
  fn test_parse_degen3() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/degen3"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_capri() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/capri"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_grow22() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/grow22"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_etamacro() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/etamacro"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_ship08l() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/ship08l"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_afiro() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/afiro"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_degen2() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/degen2"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_boeing2() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/boeing2"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_fit1d() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/fit1d"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_scfxm2() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/scfxm2"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_sctap3() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/sctap3"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_fit1p() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/fit1p"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_pilot() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/pilot"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_fit2d() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/fit2d"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_sctap2() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/sctap2"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_scfxm3() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/scfxm3"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_brandy() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/brandy"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_greenbea() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/greenbea"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_tuff() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/tuff"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_sc50a() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/sc50a"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_vtp_base() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/vtp.base"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_pilotnov() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/pilotnov"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_ship12s() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/ship12s"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_seba() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/seba"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_fffff800() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/fffff800"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_israel() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/israel"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_perold() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/perold"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_pilot4() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/pilot4"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_scagr7() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/scagr7"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_bandm() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/bandm"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_bnl1() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/bnl1"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_stocfor1() -> Result<()> {
    insta::assert_yaml_snapshot!(parse::<f32>(include_str!(
      "../tests/data/netlib/stocfor1"
    ))?);
    Ok(())
  }
}
