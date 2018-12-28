use oatie::doc::*;
use oatie::rtf::*;
use oatie::*;
use serde_json;
use oatie::deserialize::v1;

#[test]
fn test_docserialize_roundtrip_ron() {
    let start = doc_span![DocChars("birds snakes and aeroplanes")];
    let res = ron::ser::to_string(&start).unwrap();
    println!("re {:?}", res);
    let res2: Vec<DocElement<RtfSchema>> = ron::de::from_str(&res).unwrap();
    println!("re {:?}", res2);
    assert_eq!(start, res2);
    // assert_eq!(res, "[DocChars(\"birds snakes and aeroplanes\"),]");
    eprintln!();

    // TODO test link serialization
    let start = doc_span![DocChars("birds snakes and aeroplanes", {
        RtfStyle::Bold /*, Style::Link => Some("Wow".to_string()) */
    })];
    let res = ron::ser::to_string(&start).unwrap();
    println!("re {:?}", res);
    let res2: Vec<DocElement<RtfSchema>> = ron::de::from_str(&res).unwrap();
    assert_eq!(start, res2);
    println!("re {:?}", res2);
    // assert_eq!(res, "[DocChars((\"birds snakes and aeroplanes\",[Bold,],)),]");
    eprintln!();
}

#[test]
fn test_docserialize_roundtrip_json() {
    let start = doc_span![DocChars("birds snakes and aeroplanes")];
    let res = serde_json::to_string(&start).unwrap();
    println!("re.....: {:?}", res);

    let res2: Vec<DocElement<RtfSchema>> = serde_json::from_str(&res).unwrap();
    println!("re {:?}", res2);
    assert_eq!(start, res2);
    eprintln!();

    // TODO test link serialization
    let start = doc_span![DocChars("birds snakes and aeroplanes", {
        RtfStyle::Bold /*, Style::Link => Some("Wow".to_string()) */
    })];
    let res = serde_json::to_string(&start).unwrap();
    println!("re {:?}", res);
    let res2: DocSpan<RtfSchema> = serde_json::from_str(&res).unwrap();
    assert_eq!(start, res2);
    println!("re {:?}", res2);
}

#[test]
fn test_docserialize_legacy() {
    let input = r#"[DocGroup({"tag":"h1",},[DocChars(["dsdfsdno",],[Normie,],),],)]"#;
    let _res: DocSpan<RtfSchema> = v1::docspan_ron(input).unwrap();

    let input = r#"[DocChars("dsdfsdno"),]"#;
    let _res: DocSpan<RtfSchema> = v1::docspan_ron(&input).unwrap();

    let input = r#"[DocGroup({"tag":"h1",},[DocChars("home"),],),DocGroup({"tag":"p",},[DocChars("SANDBOX"),],),]"#;
    let _res: DocSpan<RtfSchema> = v1::docspan_ron(&input).unwrap();
}