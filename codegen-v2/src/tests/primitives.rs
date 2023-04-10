use crate::grammar::{
    GEof, GFuncName, GFunctionDecl, GHeaderInclude, GMarker, GNonAlphanumeric,
    GNonAlphanumericItem, GParamItem, GParamName, GPrimitive, GSeparator, GSeparatorItem, GStruct,
    GTypeCategory, ParseTree,
};
use crate::reader::Reader;

#[test]
fn test_primitives() {
    let driver = Reader::from("int");
    let der = GPrimitive::derive(driver).unwrap();
    assert_eq!(der.derived, GPrimitive::Int);

    let driver = Reader::from("bool");
    let der = GPrimitive::derive(driver).unwrap();
    assert_eq!(der.derived, GPrimitive::Bool);

    let driver = Reader::from("char\n");
    let der = GPrimitive::derive(driver).unwrap();
    assert_eq!(der.derived, GPrimitive::Char);
}