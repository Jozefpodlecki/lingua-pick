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
        name: "data_language_feature",
        sql: include_str!("003_data_language_feature.sql"),
    },
    Migration {
        version: 4,
        name: "data_exercise_category",
        sql: include_str!("004_data_exercise_category.sql"),
    },
    Migration {
        version: 5,
        name: "data_exercise_definition",
        sql: include_str!("005_data_exercise_definition.sql"),
    },
    Migration {
        version: 6,
        name: "data_language_exercise",
        sql: include_str!("006_data_language_exercise.sql"),
    },
    Migration {
        version: 7,
        name: "data_pedagogy",
        sql: include_str!("007_data_pedagogy.sql"),
    },
    Migration {
        version: 8,
        name: "data_script",
        sql: include_str!("008_data_script.sql"),
    },
    Migration {
        version: 9,
        name: "data_topic",
        sql: include_str!("009_data_topic.sql"),
    },
    Migration {
        version: 10,
        name: "data_skill",
        sql: include_str!("010_data_skill.sql"),
    },
    Migration {
        version: 100,
        name: "data_concept_pt-BR",
        sql: include_str!("100_data_concept_pt-BR.sql"),
    },
    Migration {
        version: 101,
        name: "data_concept_en-US",
        sql: include_str!("101_data_concept_en-US.sql"),
    },
    Migration {
        version: 102,
        name: "data_concept_en-GB",
        sql: include_str!("102_data_concept_en-GB.sql"),
    },
    Migration {
        version: 103,
        name: "data_concept_es-ES",
        sql: include_str!("103_data_concept_es-ES.sql"),
    },
    Migration {
        version: 104,
        name: "data_concept_es-MX",
        sql: include_str!("104_data_concept_es-MX.sql"),
    },
    Migration {
        version: 105,
        name: "data_concept_fr-FR",
        sql: include_str!("105_data_concept_fr-FR.sql"),
    },
    Migration {
        version: 106,
        name: "data_concept_fr-CA",
        sql: include_str!("106_data_concept_fr-CA.sql"),
    },
    Migration {
        version: 107,
        name: "data_concept_de-DE",
        sql: include_str!("107_data_concept_de-DE.sql"),
    },
    Migration {
        version: 108,
        name: "data_concept_it-IT",
        sql: include_str!("108_data_concept_it-IT.sql"),
    },
    Migration {
        version: 109,
        name: "data_concept_ja-JP",
        sql: include_str!("109_data_concept_ja-JP.sql"),
    },
    Migration {
        version: 110,
        name: "data_concept_ko-KR",
        sql: include_str!("110_data_concept_ko-KR.sql"),
    },
    Migration {
        version: 111,
        name: "data_concept_zh-Hans-CN",
        sql: include_str!("111_data_concept_zh-Hans-CN.sql"),
    },
    Migration {
        version: 112,
        name: "data_concept_pt-PT",
        sql: include_str!("112_data_concept_pt-PT.sql"),
    },
    Migration {
        version: 113,
        name: "data_concept_en-AU",
        sql: include_str!("113_data_concept_en-AU.sql"),
    },
    Migration {
        version: 114,
        name: "data_concept_en-CA",
        sql: include_str!("114_data_concept_en-CA.sql"),
    },
    Migration {
        version: 115,
        name: "data_concept_es-AR",
        sql: include_str!("115_data_concept_es-AR.sql"),
    },
    Migration {
        version: 116,
        name: "data_concept_es-CO",
        sql: include_str!("116_data_concept_es-CO.sql"),
    },
    Migration {
        version: 117,
        name: "data_concept_de-AT",
        sql: include_str!("117_data_concept_de-AT.sql"),
    },
    Migration {
        version: 118,
        name: "data_concept_de-CH",
        sql: include_str!("118_data_concept_de-CH.sql"),
    },
    Migration {
        version: 119,
        name: "data_concept_nl-NL",
        sql: include_str!("119_data_concept_nl-NL.sql"),
    },
    Migration {
        version: 120,
        name: "data_concept_nl-BE",
        sql: include_str!("120_data_concept_nl-BE.sql"),
    },
    Migration {
        version: 121,
        name: "data_concept_sv-SE",
        sql: include_str!("121_data_concept_sv-SE.sql"),
    },
    Migration {
        version: 122,
        name: "data_concept_nb-NO",
        sql: include_str!("122_data_concept_nb-NO.sql"),
    },
    Migration {
        version: 123,
        name: "data_concept_nn-NO",
        sql: include_str!("123_data_concept_nn-NO.sql"),
    },
    Migration {
        version: 124,
        name: "data_concept_da-DK",
        sql: include_str!("124_data_concept_da-DK.sql"),
    },
    Migration {
        version: 125,
        name: "data_concept_fi-FI",
        sql: include_str!("125_data_concept_fi-FI.sql"),
    },
    Migration {
        version: 126,
        name: "data_concept_is-IS",
        sql: include_str!("126_data_concept_is-IS.sql"),
    },
    Migration {
        version: 127,
        name: "data_concept_fo-FO",
        sql: include_str!("127_data_concept_fo-FO.sql"),
    },
    Migration {
        version: 128,
        name: "data_concept_pl-PL",
        sql: include_str!("128_data_concept_pl-PL.sql"),
    },
    Migration {
        version: 129,
        name: "data_concept_cs-CZ",
        sql: include_str!("129_data_concept_cs-CZ.sql"),
    },
    Migration {
        version: 130,
        name: "data_concept_sk-SK",
        sql: include_str!("130_data_concept_sk-SK.sql"),
    },
    Migration {
        version: 131,
        name: "data_concept_hu-HU",
        sql: include_str!("131_data_concept_hu-HU.sql"),
    },
    Migration {
        version: 132,
        name: "data_concept_ro-RO",
        sql: include_str!("132_data_concept_ro-RO.sql"),
    },
    Migration {
        version: 133,
        name: "data_concept_bg-BG",
        sql: include_str!("133_data_concept_bg-BG.sql"),
    },
    Migration {
        version: 134,
        name: "data_concept_hr-HR",
        sql: include_str!("134_data_concept_hr-HR.sql"),
    },
    Migration {
        version: 135,
        name: "data_concept_sr-RS",
        sql: include_str!("135_data_concept_sr-RS.sql"),
    },
    Migration {
        version: 136,
        name: "data_concept_bs-BA",
        sql: include_str!("136_data_concept_bs-BA.sql"),
    },
    Migration {
        version: 137,
        name: "data_concept_sl-SI",
        sql: include_str!("137_data_concept_sl-SI.sql"),
    },
    Migration {
        version: 138,
        name: "data_concept_mk-MK",
        sql: include_str!("138_data_concept_mk-MK.sql"),
    },
    Migration {
        version: 139,
        name: "data_concept_sq-AL",
        sql: include_str!("139_data_concept_sq-AL.sql"),
    },
    Migration {
        version: 140,
        name: "data_concept_el-GR",
        sql: include_str!("140_data_concept_el-GR.sql"),
    },
    Migration {
        version: 141,
        name: "data_concept_tr-TR",
        sql: include_str!("141_data_concept_tr-TR.sql"),
    },
    Migration {
        version: 142,
        name: "data_concept_ru-RU",
        sql: include_str!("142_data_concept_ru-RU.sql"),
    },
    Migration {
        version: 143,
        name: "data_concept_uk-UA",
        sql: include_str!("143_data_concept_uk-UA.sql"),
    },
    Migration {
        version: 144,
        name: "data_concept_be-BY",
        sql: include_str!("144_data_concept_be-BY.sql"),
    },
    Migration {
        version: 145,
        name: "data_concept_lt-LT",
        sql: include_str!("145_data_concept_lt-LT.sql"),
    },
    Migration {
        version: 146,
        name: "data_concept_lv-LV",
        sql: include_str!("146_data_concept_lv-LV.sql"),
    },
    Migration {
        version: 147,
        name: "data_concept_et-EE",
        sql: include_str!("147_data_concept_et-EE.sql"),
    },
    Migration {
        version: 148,
        name: "data_concept_ga-IE",
        sql: include_str!("148_data_concept_ga-IE.sql"),
    },
    Migration {
        version: 149,
        name: "data_concept_gd-GB",
        sql: include_str!("149_data_concept_gd-GB.sql"),
    },
    Migration {
        version: 150,
        name: "data_concept_cy-GB",
        sql: include_str!("150_data_concept_cy-GB.sql"),
    },
    Migration {
        version: 151,
        name: "data_concept_mt-MT",
        sql: include_str!("151_data_concept_mt-MT.sql"),
    },
    Migration {
        version: 152,
        name: "data_concept_ca-ES",
        sql: include_str!("152_data_concept_ca-ES.sql"),
    },
    Migration {
        version: 153,
        name: "data_concept_eu-ES",
        sql: include_str!("153_data_concept_eu-ES.sql"),
    },
    Migration {
        version: 154,
        name: "data_concept_gl-ES",
        sql: include_str!("154_data_concept_gl-ES.sql"),
    },
    Migration {
        version: 155,
        name: "data_concept_oc-FR",
        sql: include_str!("155_data_concept_oc-FR.sql"),
    },
    Migration {
        version: 156,
        name: "data_concept_lb-LU",
        sql: include_str!("156_data_concept_lb-LU.sql"),
    },
    Migration {
        version: 157,
        name: "data_concept_hy-AM",
        sql: include_str!("157_data_concept_hy-AM.sql"),
    },
    Migration {
        version: 158,
        name: "data_concept_ka-GE",
        sql: include_str!("158_data_concept_ka-GE.sql"),
    },
    Migration {
        version: 159,
        name: "data_concept_az-AZ",
        sql: include_str!("159_data_concept_az-AZ.sql"),
    },
    Migration {
        version: 160,
        name: "data_concept_kk-KZ",
        sql: include_str!("160_data_concept_kk-KZ.sql"),
    },
    Migration {
        version: 161,
        name: "data_concept_uz-UZ",
        sql: include_str!("161_data_concept_uz-UZ.sql"),
    },
    Migration {
        version: 162,
        name: "data_concept_ky-KG",
        sql: include_str!("162_data_concept_ky-KG.sql"),
    },
    Migration {
        version: 163,
        name: "data_concept_tg-TJ",
        sql: include_str!("163_data_concept_tg-TJ.sql"),
    },
    Migration {
        version: 164,
        name: "data_concept_mn-MN",
        sql: include_str!("164_data_concept_mn-MN.sql"),
    },
    Migration {
        version: 165,
        name: "data_concept_zh-Hant-TW",
        sql: include_str!("165_data_concept_zh-Hant-TW.sql"),
    },
    Migration {
        version: 166,
        name: "data_concept_yue-Hant-HK",
        sql: include_str!("166_data_concept_yue-Hant-HK.sql"),
    },
    Migration {
        version: 167,
        name: "data_concept_vi-VN",
        sql: include_str!("167_data_concept_vi-VN.sql"),
    },
    Migration {
        version: 168,
        name: "data_concept_th-TH",
        sql: include_str!("168_data_concept_th-TH.sql"),
    },
    Migration {
        version: 169,
        name: "data_concept_lo-LA",
        sql: include_str!("169_data_concept_lo-LA.sql"),
    },
    Migration {
        version: 170,
        name: "data_concept_km-KH",
        sql: include_str!("170_data_concept_km-KH.sql"),
    },
    Migration {
        version: 171,
        name: "data_concept_my-MM",
        sql: include_str!("171_data_concept_my-MM.sql"),
    },
    Migration {
        version: 172,
        name: "data_concept_id-ID",
        sql: include_str!("172_data_concept_id-ID.sql"),
    },
    Migration {
        version: 173,
        name: "data_concept_ms-MY",
        sql: include_str!("173_data_concept_ms-MY.sql"),
    },
    Migration {
        version: 174,
        name: "data_concept_fil-PH",
        sql: include_str!("174_data_concept_fil-PH.sql"),
    },
    Migration {
        version: 175,
        name: "data_concept_jv-ID",
        sql: include_str!("175_data_concept_jv-ID.sql"),
    },
    Migration {
        version: 176,
        name: "data_concept_su-ID",
        sql: include_str!("176_data_concept_su-ID.sql"),
    },
    Migration {
        version: 177,
        name: "data_concept_hi-IN",
        sql: include_str!("177_data_concept_hi-IN.sql"),
    },
    Migration {
        version: 178,
        name: "data_concept_ur-PK",
        sql: include_str!("178_data_concept_ur-PK.sql"),
    },
    Migration {
        version: 179,
        name: "data_concept_bn-BD",
        sql: include_str!("179_data_concept_bn-BD.sql"),
    },
    Migration {
        version: 180,
        name: "data_concept_pa-IN",
        sql: include_str!("180_data_concept_pa-IN.sql"),
    },
    Migration {
        version: 181,
        name: "data_concept_gu-IN",
        sql: include_str!("181_data_concept_gu-IN.sql"),
    },
    Migration {
        version: 182,
        name: "data_concept_mr-IN",
        sql: include_str!("182_data_concept_mr-IN.sql"),
    },
    Migration {
        version: 183,
        name: "data_concept_ta-IN",
        sql: include_str!("183_data_concept_ta-IN.sql"),
    },
    Migration {
        version: 184,
        name: "data_concept_te-IN",
        sql: include_str!("184_data_concept_te-IN.sql"),
    },
    Migration {
        version: 185,
        name: "data_concept_kn-IN",
        sql: include_str!("185_data_concept_kn-IN.sql"),
    },
    Migration {
        version: 186,
        name: "data_concept_ml-IN",
        sql: include_str!("186_data_concept_ml-IN.sql"),
    },
    Migration {
        version: 187,
        name: "data_concept_si-LK",
        sql: include_str!("187_data_concept_si-LK.sql"),
    },
    Migration {
        version: 188,
        name: "data_concept_ne-NP",
        sql: include_str!("188_data_concept_ne-NP.sql"),
    },
    Migration {
        version: 189,
        name: "data_concept_fa-IR",
        sql: include_str!("189_data_concept_fa-IR.sql"),
    },
    Migration {
        version: 190,
        name: "data_concept_ps-AF",
        sql: include_str!("190_data_concept_ps-AF.sql"),
    },
    Migration {
        version: 191,
        name: "data_concept_he-IL",
        sql: include_str!("191_data_concept_he-IL.sql"),
    },
    Migration {
        version: 192,
        name: "data_concept_ar",
        sql: include_str!("192_data_concept_ar.sql"),
    },
    Migration {
        version: 193,
        name: "data_concept_ar-EG",
        sql: include_str!("193_data_concept_ar-EG.sql"),
    },
    Migration {
        version: 194,
        name: "data_concept_sw",
        sql: include_str!("194_data_concept_sw.sql"),
    },
    Migration {
        version: 195,
        name: "data_concept_af-ZA",
        sql: include_str!("195_data_concept_af-ZA.sql"),
    },
    Migration {
        version: 196,
        name: "data_concept_zu-ZA",
        sql: include_str!("196_data_concept_zu-ZA.sql"),
    },
    Migration {
        version: 197,
        name: "data_concept_xh-ZA",
        sql: include_str!("197_data_concept_xh-ZA.sql"),
    },
    Migration {
        version: 198,
        name: "data_concept_am-ET",
        sql: include_str!("198_data_concept_am-ET.sql"),
    },
    Migration {
        version: 199,
        name: "data_concept_so-SO",
        sql: include_str!("199_data_concept_so-SO.sql"),
    },
    Migration {
        version: 200,
        name: "data_concept_es-CL",
        sql: include_str!("200_data_concept_es-CL.sql"),
    },
    Migration {
        version: 201,
        name: "data_concept_es-PE",
        sql: include_str!("201_data_concept_es-PE.sql"),
    },
    Migration {
        version: 202,
        name: "data_concept_es-VE",
        sql: include_str!("202_data_concept_es-VE.sql"),
    },
    Migration {
        version: 203,
        name: "data_concept_es-UY",
        sql: include_str!("203_data_concept_es-UY.sql"),
    },
    Migration {
        version: 204,
        name: "data_concept_es-EC",
        sql: include_str!("204_data_concept_es-EC.sql"),
    },
    Migration {
        version: 205,
        name: "data_concept_es-BO",
        sql: include_str!("205_data_concept_es-BO.sql"),
    },
    Migration {
        version: 206,
        name: "data_concept_es-PY",
        sql: include_str!("206_data_concept_es-PY.sql"),
    },
    Migration {
        version: 207,
        name: "data_concept_es-CR",
        sql: include_str!("207_data_concept_es-CR.sql"),
    },
    Migration {
        version: 208,
        name: "data_concept_es-PA",
        sql: include_str!("208_data_concept_es-PA.sql"),
    },
    Migration {
        version: 209,
        name: "data_concept_es-GT",
        sql: include_str!("209_data_concept_es-GT.sql"),
    },
    Migration {
        version: 210,
        name: "data_concept_es-HN",
        sql: include_str!("210_data_concept_es-HN.sql"),
    },
    Migration {
        version: 211,
        name: "data_concept_es-SV",
        sql: include_str!("211_data_concept_es-SV.sql"),
    },
    Migration {
        version: 212,
        name: "data_concept_es-NI",
        sql: include_str!("212_data_concept_es-NI.sql"),
    },
    Migration {
        version: 213,
        name: "data_concept_es-CU",
        sql: include_str!("213_data_concept_es-CU.sql"),
    },
    Migration {
        version: 214,
        name: "data_concept_es-DO",
        sql: include_str!("214_data_concept_es-DO.sql"),
    },
    Migration {
        version: 215,
        name: "data_concept_es-PR",
        sql: include_str!("215_data_concept_es-PR.sql"),
    },
    Migration {
        version: 216,
        name: "data_concept_en-NZ",
        sql: include_str!("216_data_concept_en-NZ.sql"),
    },
    Migration {
        version: 217,
        name: "data_concept_en-IE",
        sql: include_str!("217_data_concept_en-IE.sql"),
    },
    Migration {
        version: 218,
        name: "data_concept_en-ZA",
        sql: include_str!("218_data_concept_en-ZA.sql"),
    },
    Migration {
        version: 219,
        name: "data_concept_en-IN",
        sql: include_str!("219_data_concept_en-IN.sql"),
    },
    Migration {
        version: 220,
        name: "data_concept_en-SG",
        sql: include_str!("220_data_concept_en-SG.sql"),
    },
    Migration {
        version: 221,
        name: "data_concept_en-PH",
        sql: include_str!("221_data_concept_en-PH.sql"),
    },
    Migration {
        version: 222,
        name: "data_concept_en-NG",
        sql: include_str!("222_data_concept_en-NG.sql"),
    },
    Migration {
        version: 223,
        name: "data_concept_en-KE",
        sql: include_str!("223_data_concept_en-KE.sql"),
    },
    Migration {
        version: 224,
        name: "data_concept_en-JM",
        sql: include_str!("224_data_concept_en-JM.sql"),
    },
    Migration {
        version: 225,
        name: "data_concept_en-TT",
        sql: include_str!("225_data_concept_en-TT.sql"),
    },
    Migration {
        version: 226,
        name: "data_concept_fr-BE",
        sql: include_str!("226_data_concept_fr-BE.sql"),
    },
    Migration {
        version: 227,
        name: "data_concept_fr-CH",
        sql: include_str!("227_data_concept_fr-CH.sql"),
    },
    Migration {
        version: 228,
        name: "data_concept_fr-LU",
        sql: include_str!("228_data_concept_fr-LU.sql"),
    },
    Migration {
        version: 229,
        name: "data_concept_fr-SN",
        sql: include_str!("229_data_concept_fr-SN.sql"),
    },
    Migration {
        version: 230,
        name: "data_concept_fr-CI",
        sql: include_str!("230_data_concept_fr-CI.sql"),
    },
    Migration {
        version: 231,
        name: "data_concept_fr-CM",
        sql: include_str!("231_data_concept_fr-CM.sql"),
    },
    Migration {
        version: 232,
        name: "data_concept_fr-CD",
        sql: include_str!("232_data_concept_fr-CD.sql"),
    },
    Migration {
        version: 233,
        name: "data_concept_fr-HT",
        sql: include_str!("233_data_concept_fr-HT.sql"),
    },
    Migration {
        version: 234,
        name: "data_concept_pt-AO",
        sql: include_str!("234_data_concept_pt-AO.sql"),
    },
    Migration {
        version: 235,
        name: "data_concept_pt-MZ",
        sql: include_str!("235_data_concept_pt-MZ.sql"),
    },
    Migration {
        version: 236,
        name: "data_concept_pt-CV",
        sql: include_str!("236_data_concept_pt-CV.sql"),
    },
    Migration {
        version: 237,
        name: "data_concept_pt-GW",
        sql: include_str!("237_data_concept_pt-GW.sql"),
    },
    Migration {
        version: 238,
        name: "data_concept_pt-ST",
        sql: include_str!("238_data_concept_pt-ST.sql"),
    },
    Migration {
        version: 239,
        name: "data_concept_de-LU",
        sql: include_str!("239_data_concept_de-LU.sql"),
    },
    Migration {
        version: 240,
        name: "data_concept_de-LI",
        sql: include_str!("240_data_concept_de-LI.sql"),
    },
    Migration {
        version: 241,
        name: "data_concept_it-CH",
        sql: include_str!("241_data_concept_it-CH.sql"),
    },
    Migration {
        version: 242,
        name: "data_concept_nl-SR",
        sql: include_str!("242_data_concept_nl-SR.sql"),
    },
    Migration {
        version: 243,
        name: "data_concept_ar-SA",
        sql: include_str!("243_data_concept_ar-SA.sql"),
    },
    Migration {
        version: 244,
        name: "data_concept_ar-MA",
        sql: include_str!("244_data_concept_ar-MA.sql"),
    },
    Migration {
        version: 245,
        name: "data_concept_ar-DZ",
        sql: include_str!("245_data_concept_ar-DZ.sql"),
    },
    Migration {
        version: 246,
        name: "data_concept_ar-TN",
        sql: include_str!("246_data_concept_ar-TN.sql"),
    },
    Migration {
        version: 247,
        name: "data_concept_ar-LB",
        sql: include_str!("247_data_concept_ar-LB.sql"),
    },
    Migration {
        version: 248,
        name: "data_concept_sw-KE",
        sql: include_str!("248_data_concept_sw-KE.sql"),
    },
    Migration {
        version: 249,
        name: "data_concept_sw-TZ",
        sql: include_str!("249_data_concept_sw-TZ.sql"),
    },
];
