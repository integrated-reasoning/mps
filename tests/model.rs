mod tests {
  use color_eyre::Result;
  use mps::model::Model;
  use mps::Parser;

  #[test]
  fn test_model_from_agg() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/agg"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_ship04l() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/ship04l"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_d2q06c() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/d2q06c"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_e226() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/e226"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_nl25fv47() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/25fv47"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_bore3d() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/bore3d"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_ganges() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/ganges"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_adlittle() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/adlittle"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_forplan() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/forplan"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_sc205() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/sc205"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_nl80bau3b() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/80bau3b"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_scrs8() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/scrs8"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_wood1p() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/wood1p"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_boeing1() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/boeing1"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_kb2() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/kb2"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_ship08s() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/ship08s"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_scfxm1() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/scfxm1"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_agg2() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/agg2"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_finnis() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/finnis"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_dfl001() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/dfl001"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_pilot87() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/pilot87"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_sctap1() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/sctap1"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_agg3() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/agg3"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_grow7() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/grow7"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_scorpion() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/scorpion"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_maros() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/maros"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_shell() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/shell"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_greenbeb() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/greenbeb"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_sc50b() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/sc50b"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_recipe() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/recipe"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_sierra() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/sierra"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_scagr25() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/scagr25"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_modszk1() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/modszk1"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_ship12l() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/ship12l"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_stair() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/stair"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_cycle() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/cycle"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_sc105() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/sc105"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_pilot_ja() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/pilot.ja"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_beaconfd() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/beaconfd"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_czprob() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/czprob"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_pilot_we() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/pilot.we"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_standgub() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/standgub"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_standmps() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/standmps"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_scsd8() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/scsd8"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_woodw() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/woodw"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_scsd6() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/scsd6"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_scsd1() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/scsd1"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_share2b() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/share2b"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_gfrd_pnc() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/gfrd-pnc"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_bnl2() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/bnl2"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_stocfor2() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/stocfor2"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_nesm() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/nesm"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_share1b() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/share1b"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_ship04s() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/ship04s"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_grow15() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/grow15"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_maros_r7() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/maros-r7"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_blend() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/blend"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_lotfi() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/lotfi"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_standata() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/standata"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_d6cube() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/d6cube"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_degen3() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/degen3"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_capri() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/capri"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_grow22() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/grow22"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_etamacro() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/etamacro"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_ship08l() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/ship08l"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_afiro() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/afiro"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_degen2() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/degen2"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_boeing2() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/boeing2"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_fit1d() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/fit1d"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_scfxm2() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/scfxm2"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_sctap3() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/sctap3"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_fit1p() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/fit1p"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_pilot() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/pilot"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_fit2d() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/fit2d"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_sctap2() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/sctap2"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_scfxm3() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/scfxm3"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_brandy() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/brandy"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_greenbea() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/greenbea"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_tuff() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/tuff"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_sc50a() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/sc50a"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_vtp_base() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/vtp.base"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_pilotnov() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/pilotnov"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_ship12s() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/ship12s"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_seba() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/seba"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_fffff800() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/fffff800"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_israel() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/israel"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_perold() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/perold"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_model_from_pilot4() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/pilot4"))?;
    let model = Model::try_from(parsed)?;
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }

  #[test]
  fn test_parse_scagr7() -> Result<()> {
    insta::assert_yaml_snapshot!(Parser::<f32>::parse(include_str!(
      "../tests/data/netlib/scagr7"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_bandm() -> Result<()> {
    insta::assert_yaml_snapshot!(Parser::<f32>::parse(include_str!(
      "../tests/data/netlib/bandm"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_bnl1() -> Result<()> {
    insta::assert_yaml_snapshot!(Parser::<f32>::parse(include_str!(
      "../tests/data/netlib/bnl1"
    ))?);
    Ok(())
  }

  #[test]
  fn test_parse_stocfor1() -> Result<()> {
    insta::assert_yaml_snapshot!(Parser::<f32>::parse(include_str!(
      "../tests/data/netlib/stocfor1"
    ))?);
    Ok(())
  }
}
