use oatie::deserialize::{
    v1,
    v2,
};
use oatie::doc::*;
use oatie::rtf::*;
use oatie::*;
use serde_json;

#[test]
fn test_docserialize_roundtrip_ron() {
    let start = doc![DocText("birds snakes and aeroplanes")];
    let res = ron::ser::to_string(&start).unwrap();
    println!("re {:?}", res);
    let res2: Doc<RtfSchema> = ron::de::from_str(&res).unwrap();
    println!("re {:?}", res2);
    assert_eq!(start, res2);
    // assert_eq!(res, "[DocText(\"birds snakes and aeroplanes\"),]");
    eprintln!();

    // TODO test link serialization
    let start = doc![DocText(
        {
            RtfStyle::Bold /*, Style::Link => Some("Wow".to_string()) */
        },
        "birds snakes and aeroplanes"
    )];
    let res = ron::ser::to_string(&start).unwrap();
    println!("re {:?}", res);
    let res2: Doc<RtfSchema> = ron::de::from_str(&res).unwrap();
    assert_eq!(start, res2);
    println!("re {:?}", res2);
    // assert_eq!(res, "[DocText((\"birds snakes and aeroplanes\",[Bold,],)),]");
    eprintln!();
}

#[test]
fn test_docserialize_roundtrip_json() {
    let start = doc![DocText("birds snakes and aeroplanes")];
    let res = serde_json::to_string(&start).unwrap();
    println!("re.....: {:?}", res);

    let res2: Doc<RtfSchema> = serde_json::from_str(&res).unwrap();
    println!("re {:?}", res2);
    assert_eq!(start, res2);
    eprintln!();

    // TODO test link serialization
    let start = doc![DocText(
        {
            RtfStyle::Bold /*, Style::Link => Some("Wow".to_string()) */
        },
        "birds snakes and aeroplanes"
    )];
    let res = serde_json::to_string(&start).unwrap();
    println!("re {:?}", res);
    let res2: Doc<RtfSchema> = serde_json::from_str(&res).unwrap();
    assert_eq!(start, res2);
    println!("re {:?}", res2);
}

#[test]
fn test_docserialize_v2() {
    let input = r#"[DocGroup({"tag":"h1",},[DocChars(["dsdfsdno",],[Normie,],),],)]"#;
    let _res: DocSpan<RtfSchema> = v2::docspan_ron(input).unwrap();

    let input = r#"[DocChars("dsdfsdno"),]"#;
    let _res: DocSpan<RtfSchema> = v2::docspan_ron(&input).unwrap();

    let input = r#"[DocGroup({"tag":"h1",},[DocChars("home"),],),DocGroup({"tag":"p",},[DocChars("SANDBOX"),],),]"#;
    let _res: DocSpan<RtfSchema> = v2::docspan_ron(&input).unwrap();
}

#[test]
fn test_docserialize_v1() {
    let input = r#"[DocGroup({"tag":"h1",},[DocChars( [  "hey!",{Normie:None,},]),],)]"#;
    let _res: DocSpan<RtfSchema> = v1::docspan_ron(&input).unwrap();
}

#[test]
fn test_docserialize_modern() {
    let input = r#"Doc([DocGroup(Header(1),[DocText([],"dsdfsdno"),],)])"#;
    let _res: Doc<RtfSchema> = ron::de::from_str(input).unwrap();

    let input = r#"[{"DocGroup":[{"Header":1},[{"DocText":[[],"dsdfsdno"]}]]}]"#;
    let _res: Doc<RtfSchema> = serde_json::from_str(&input).unwrap();

    let input = r#"[{"DocGroup":["Para",[{"DocText":[["Bold"],"dsdfsdno"]}]]}]"#;
    let _res: Doc<RtfSchema> = serde_json::from_str(&input).unwrap();
}
