use io::itos;

#[test]
fn itos_test(){
    let mut buf = vec![0u8; 2];
    itos(0xAA, &mut buf);
    println!("{}", str::from_utf8(&buf).unwrap());
    for i in -0xFF..0xFF{
        let mut buf = vec![0u8; 2];
        itos(i, &mut buf);
        assert_eq!(String::from_utf8(buf).unwrap(), format!("{}", i as i8));
    }
}