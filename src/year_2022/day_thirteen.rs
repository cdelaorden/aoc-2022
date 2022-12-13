
pub fn distress_signal(data: &str) {

}

#[derive(Debug, Clone)]
enum PacketData {
  Integer(u32),
  List(Vec<Box<PacketData>>),
  Empty
}
impl PartialEq for PacketData {
  fn eq(&self, other: &Self) -> bool {
      match (self, other) {
        (PacketData::Integer(x), PacketData::Integer(y)) => x == y,
        (PacketData::Empty, PacketData::Empty) => true,
        (PacketData::List(xs), PacketData::List(ys)) => {
          if xs.len() != ys.len() {
            return false
          }          
          for i in 0..xs.len() {
            if xs[i] == ys[i] {
              return false
            }
          }
          true
        },
        _ => false
      }
  }
}
type PacketPair = (PacketData, PacketData);

fn parse(data:&str) -> Vec<PacketPair> {
  let mut out = Vec::new();
  
  out
}

fn parse_contents (contents:&str) -> Vec<Box<PacketData>> {
  let mut contents = Vec::new();
  contents.push(Box::new(PacketData::Integer(1)));
  contents
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_parses_list_of_integers(){
    let res = parse_contents("[1,1,1]");
    assert_eq!(res[0], Box::new(PacketData::Integer(1)));
  }

}