use super::Migration;

pub(super) const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        name: "initial",
        sql: include_str!("001_initial.sql"),
    },
    Migration {
        version: 2,
        name: "data_language",
        sql: include_str!("002_data_language.sql"),
    },
    Migration {
        version: 3,
        name: "data_exercise_category",
        sql: include_str!("003_data_exercise_category.sql"),
    },
    Migration {
        version: 4,
        name: "data_exercise_definition",
        sql: include_str!("004_data_exercise_definition.sql"),
    },
    Migration {
        version: 5,
        name: "data_language_exercise",
        sql: include_str!("005_data_language_exercise.sql"),
    },
    Migration {
        version: 6,
        name: "data_pedagogy",
        sql: include_str!("006_data_pedagogy.sql"),
    },
    Migration {
        version: 7,
        name: "data_script",
        sql: include_str!("007_data_script.sql"),
    },
    Migration {
        version: 8,
        name: "data_topic",
        sql: include_str!("008_data_topic.sql"),
    },
    Migration {
        version: 9,
        name: "data_skill",
        sql: include_str!("009_data_skill.sql"),
    },
    Migration {
        version: 10,
        name: "data_concept_pt-BR",
        sql: include_str!("010_data_concept_pt-BR.sql"),
    },
    Migration {
        version: 11,
        name: "data_concept_en-US",
        sql: include_str!("011_data_concept_en-US.sql"),
    },
    Migration {
        version: 12,
        name: "data_concept_en-GB",
        sql: include_str!("012_data_concept_en-GB.sql"),
    },
    Migration {
        version: 13,
        name: "data_concept_es-ES",
        sql: include_str!("013_data_concept_es-ES.sql"),
    },
    Migration {
        version: 14,
        name: "data_concept_es-MX",
        sql: include_str!("014_data_concept_es-MX.sql"),
    },
    Migration {
        version: 15,
        name: "data_concept_fr-FR",
        sql: include_str!("015_data_concept_fr-FR.sql"),
    },
    Migration {
        version: 16,
        name: "data_concept_fr-CA",
        sql: include_str!("016_data_concept_fr-CA.sql"),
    },
    Migration {
        version: 17,
        name: "data_concept_de-DE",
        sql: include_str!("017_data_concept_de-DE.sql"),
    },
    Migration {
        version: 18,
        name: "data_concept_it-IT",
        sql: include_str!("018_data_concept_it-IT.sql"),
    },
    Migration {
        version: 19,
        name: "data_concept_ja-JP",
        sql: include_str!("019_data_concept_ja-JP.sql"),
    },
    Migration {
        version: 20,
        name: "data_concept_ko-KR",
        sql: include_str!("020_data_concept_ko-KR.sql"),
    },
    Migration {
        version: 21,
        name: "data_concept_zh-Hans-CN",
        sql: include_str!("021_data_concept_zh-Hans-CN.sql"),
    },
    Migration {
        version: 22,
        name: "data_concept_pt-PT",
        sql: include_str!("022_data_concept_pt-PT.sql"),
    },
    Migration {
        version: 23,
        name: "data_concept_en-AU",
        sql: include_str!("023_data_concept_en-AU.sql"),
    },
    Migration {
        version: 24,
        name: "data_concept_en-CA",
        sql: include_str!("024_data_concept_en-CA.sql"),
    },
    Migration {
        version: 25,
        name: "data_concept_es-AR",
        sql: include_str!("025_data_concept_es-AR.sql"),
    },
    Migration {
        version: 26,
        name: "data_concept_es-CO",
        sql: include_str!("026_data_concept_es-CO.sql"),
    },
    Migration {
        version: 27,
        name: "data_concept_de-AT",
        sql: include_str!("027_data_concept_de-AT.sql"),
    },
    Migration {
        version: 28,
        name: "data_concept_de-CH",
        sql: include_str!("028_data_concept_de-CH.sql"),
    },
    Migration {
        version: 29,
        name: "data_concept_nl-NL",
        sql: include_str!("029_data_concept_nl-NL.sql"),
    },
    Migration {
        version: 30,
        name: "data_concept_nl-BE",
        sql: include_str!("030_data_concept_nl-BE.sql"),
    },
    Migration {
        version: 31,
        name: "data_concept_sv-SE",
        sql: include_str!("031_data_concept_sv-SE.sql"),
    },
    Migration {
        version: 32,
        name: "data_concept_nb-NO",
        sql: include_str!("032_data_concept_nb-NO.sql"),
    },
    Migration {
        version: 33,
        name: "data_concept_nn-NO",
        sql: include_str!("033_data_concept_nn-NO.sql"),
    },
    Migration {
        version: 34,
        name: "data_concept_da-DK",
        sql: include_str!("034_data_concept_da-DK.sql"),
    },
    Migration {
        version: 35,
        name: "data_concept_fi-FI",
        sql: include_str!("035_data_concept_fi-FI.sql"),
    },
    Migration {
        version: 36,
        name: "data_concept_is-IS",
        sql: include_str!("036_data_concept_is-IS.sql"),
    },
    Migration {
        version: 37,
        name: "data_concept_fo-FO",
        sql: include_str!("037_data_concept_fo-FO.sql"),
    },
    Migration {
        version: 38,
        name: "data_concept_pl-PL",
        sql: include_str!("038_data_concept_pl-PL.sql"),
    },
    Migration {
        version: 39,
        name: "data_concept_cs-CZ",
        sql: include_str!("039_data_concept_cs-CZ.sql"),
    },
    Migration {
        version: 40,
        name: "data_concept_sk-SK",
        sql: include_str!("040_data_concept_sk-SK.sql"),
    },
    Migration {
        version: 41,
        name: "data_concept_hu-HU",
        sql: include_str!("041_data_concept_hu-HU.sql"),
    },
    Migration {
        version: 42,
        name: "data_concept_ro-RO",
        sql: include_str!("042_data_concept_ro-RO.sql"),
    },
    Migration {
        version: 43,
        name: "data_concept_bg-BG",
        sql: include_str!("043_data_concept_bg-BG.sql"),
    },
    Migration {
        version: 44,
        name: "data_concept_hr-HR",
        sql: include_str!("044_data_concept_hr-HR.sql"),
    },
    Migration {
        version: 45,
        name: "data_concept_sr-RS",
        sql: include_str!("045_data_concept_sr-RS.sql"),
    },
    Migration {
        version: 46,
        name: "data_concept_bs-BA",
        sql: include_str!("046_data_concept_bs-BA.sql"),
    },
    Migration {
        version: 47,
        name: "data_concept_sl-SI",
        sql: include_str!("047_data_concept_sl-SI.sql"),
    },
    Migration {
        version: 48,
        name: "data_concept_mk-MK",
        sql: include_str!("048_data_concept_mk-MK.sql"),
    },
    Migration {
        version: 49,
        name: "data_concept_sq-AL",
        sql: include_str!("049_data_concept_sq-AL.sql"),
    },
    Migration {
        version: 50,
        name: "data_concept_el-GR",
        sql: include_str!("050_data_concept_el-GR.sql"),
    },
    Migration {
        version: 51,
        name: "data_concept_tr-TR",
        sql: include_str!("051_data_concept_tr-TR.sql"),
    },
    Migration {
        version: 52,
        name: "data_concept_ru-RU",
        sql: include_str!("052_data_concept_ru-RU.sql"),
    },
    Migration {
        version: 53,
        name: "data_concept_uk-UA",
        sql: include_str!("053_data_concept_uk-UA.sql"),
    },
    Migration {
        version: 54,
        name: "data_concept_be-BY",
        sql: include_str!("054_data_concept_be-BY.sql"),
    },
    Migration {
        version: 55,
        name: "data_concept_lt-LT",
        sql: include_str!("055_data_concept_lt-LT.sql"),
    },
    Migration {
        version: 56,
        name: "data_concept_lv-LV",
        sql: include_str!("056_data_concept_lv-LV.sql"),
    },
    Migration {
        version: 57,
        name: "data_concept_et-EE",
        sql: include_str!("057_data_concept_et-EE.sql"),
    },
    Migration {
        version: 58,
        name: "data_concept_ga-IE",
        sql: include_str!("058_data_concept_ga-IE.sql"),
    },
    Migration {
        version: 59,
        name: "data_concept_gd-GB",
        sql: include_str!("059_data_concept_gd-GB.sql"),
    },
    Migration {
        version: 60,
        name: "data_concept_cy-GB",
        sql: include_str!("060_data_concept_cy-GB.sql"),
    },
    Migration {
        version: 61,
        name: "data_concept_mt-MT",
        sql: include_str!("061_data_concept_mt-MT.sql"),
    },
    Migration {
        version: 62,
        name: "data_concept_ca-ES",
        sql: include_str!("062_data_concept_ca-ES.sql"),
    },
    Migration {
        version: 63,
        name: "data_concept_eu-ES",
        sql: include_str!("063_data_concept_eu-ES.sql"),
    },
    Migration {
        version: 64,
        name: "data_concept_gl-ES",
        sql: include_str!("064_data_concept_gl-ES.sql"),
    },
    Migration {
        version: 65,
        name: "data_concept_oc-FR",
        sql: include_str!("065_data_concept_oc-FR.sql"),
    },
    Migration {
        version: 66,
        name: "data_concept_lb-LU",
        sql: include_str!("066_data_concept_lb-LU.sql"),
    },
    Migration {
        version: 67,
        name: "data_concept_hy-AM",
        sql: include_str!("067_data_concept_hy-AM.sql"),
    },
    Migration {
        version: 68,
        name: "data_concept_ka-GE",
        sql: include_str!("068_data_concept_ka-GE.sql"),
    },
    Migration {
        version: 69,
        name: "data_concept_az-AZ",
        sql: include_str!("069_data_concept_az-AZ.sql"),
    },
    Migration {
        version: 70,
        name: "data_concept_kk-KZ",
        sql: include_str!("070_data_concept_kk-KZ.sql"),
    },
    Migration {
        version: 71,
        name: "data_concept_uz-UZ",
        sql: include_str!("071_data_concept_uz-UZ.sql"),
    },
    Migration {
        version: 72,
        name: "data_concept_ky-KG",
        sql: include_str!("072_data_concept_ky-KG.sql"),
    },
    Migration {
        version: 73,
        name: "data_concept_tg-TJ",
        sql: include_str!("073_data_concept_tg-TJ.sql"),
    },
    Migration {
        version: 74,
        name: "data_concept_mn-MN",
        sql: include_str!("074_data_concept_mn-MN.sql"),
    },
    Migration {
        version: 75,
        name: "data_concept_zh-Hant-TW",
        sql: include_str!("075_data_concept_zh-Hant-TW.sql"),
    },
    Migration {
        version: 76,
        name: "data_concept_yue-Hant-HK",
        sql: include_str!("076_data_concept_yue-Hant-HK.sql"),
    },
    Migration {
        version: 77,
        name: "data_concept_vi-VN",
        sql: include_str!("077_data_concept_vi-VN.sql"),
    },
    Migration {
        version: 78,
        name: "data_concept_th-TH",
        sql: include_str!("078_data_concept_th-TH.sql"),
    },
    Migration {
        version: 79,
        name: "data_concept_lo-LA",
        sql: include_str!("079_data_concept_lo-LA.sql"),
    },
    Migration {
        version: 80,
        name: "data_concept_km-KH",
        sql: include_str!("080_data_concept_km-KH.sql"),
    },
    Migration {
        version: 81,
        name: "data_concept_my-MM",
        sql: include_str!("081_data_concept_my-MM.sql"),
    },
    Migration {
        version: 82,
        name: "data_concept_id-ID",
        sql: include_str!("082_data_concept_id-ID.sql"),
    },
    Migration {
        version: 83,
        name: "data_concept_ms-MY",
        sql: include_str!("083_data_concept_ms-MY.sql"),
    },
    Migration {
        version: 84,
        name: "data_concept_fil-PH",
        sql: include_str!("084_data_concept_fil-PH.sql"),
    },
    Migration {
        version: 85,
        name: "data_concept_jv-ID",
        sql: include_str!("085_data_concept_jv-ID.sql"),
    },
    Migration {
        version: 86,
        name: "data_concept_su-ID",
        sql: include_str!("086_data_concept_su-ID.sql"),
    },
    Migration {
        version: 87,
        name: "data_concept_hi-IN",
        sql: include_str!("087_data_concept_hi-IN.sql"),
    },
    Migration {
        version: 88,
        name: "data_concept_ur-PK",
        sql: include_str!("088_data_concept_ur-PK.sql"),
    },
    Migration {
        version: 89,
        name: "data_concept_bn-BD",
        sql: include_str!("089_data_concept_bn-BD.sql"),
    },
    Migration {
        version: 90,
        name: "data_concept_pa-IN",
        sql: include_str!("090_data_concept_pa-IN.sql"),
    },
    Migration {
        version: 91,
        name: "data_concept_gu-IN",
        sql: include_str!("091_data_concept_gu-IN.sql"),
    },
    Migration {
        version: 92,
        name: "data_concept_mr-IN",
        sql: include_str!("092_data_concept_mr-IN.sql"),
    },
    Migration {
        version: 93,
        name: "data_concept_ta-IN",
        sql: include_str!("093_data_concept_ta-IN.sql"),
    },
    Migration {
        version: 94,
        name: "data_concept_te-IN",
        sql: include_str!("094_data_concept_te-IN.sql"),
    },
    Migration {
        version: 95,
        name: "data_concept_kn-IN",
        sql: include_str!("095_data_concept_kn-IN.sql"),
    },
    Migration {
        version: 96,
        name: "data_concept_ml-IN",
        sql: include_str!("096_data_concept_ml-IN.sql"),
    },
    Migration {
        version: 97,
        name: "data_concept_si-LK",
        sql: include_str!("097_data_concept_si-LK.sql"),
    },
    Migration {
        version: 98,
        name: "data_concept_ne-NP",
        sql: include_str!("098_data_concept_ne-NP.sql"),
    },
    Migration {
        version: 99,
        name: "data_concept_fa-IR",
        sql: include_str!("099_data_concept_fa-IR.sql"),
    },
    Migration {
        version: 100,
        name: "data_concept_ps-AF",
        sql: include_str!("100_data_concept_ps-AF.sql"),
    },
    Migration {
        version: 101,
        name: "data_concept_he-IL",
        sql: include_str!("101_data_concept_he-IL.sql"),
    },
    Migration {
        version: 102,
        name: "data_concept_ar",
        sql: include_str!("102_data_concept_ar.sql"),
    },
    Migration {
        version: 103,
        name: "data_concept_ar-EG",
        sql: include_str!("103_data_concept_ar-EG.sql"),
    },
    Migration {
        version: 104,
        name: "data_concept_sw",
        sql: include_str!("104_data_concept_sw.sql"),
    },
    Migration {
        version: 105,
        name: "data_concept_af-ZA",
        sql: include_str!("105_data_concept_af-ZA.sql"),
    },
    Migration {
        version: 106,
        name: "data_concept_zu-ZA",
        sql: include_str!("106_data_concept_zu-ZA.sql"),
    },
    Migration {
        version: 107,
        name: "data_concept_xh-ZA",
        sql: include_str!("107_data_concept_xh-ZA.sql"),
    },
    Migration {
        version: 108,
        name: "data_concept_am-ET",
        sql: include_str!("108_data_concept_am-ET.sql"),
    },
    Migration {
        version: 109,
        name: "data_concept_so-SO",
        sql: include_str!("109_data_concept_so-SO.sql"),
    },
];
