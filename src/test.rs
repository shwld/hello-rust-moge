fn moge(name: &str) -> String {
  let result = format!("{}_{}", name, "hoge");
  result
}

pub fn perform(count: u8, name: &str) {
  for _ in 0..count {
      println!("Hello {}!", moge(name))
  }
}

#[test]
fn run_moge() {
  let str = moge("hoge");
  assert_eq!(str, "hoge_hoge");
}
