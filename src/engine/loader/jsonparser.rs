use std::fs;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum NodeType{
  String,
  Number,
  Bolean,
  Other,
}

pub struct JsonF{
  pub name: String,
  pub strval: String,
  pub numeral_val: f32,
  pub bolean: bool,
  pub index: u32,
  pub indexed: bool,
  pub other_nodes: Vec<JsonF>,
  pub node_type: NodeType,
}

impl JsonF {
  pub fn get_node(&mut self, path: Vec<usize>) -> &mut JsonF{
    if path.len() < 1{
      return self;
    }
    let mut lp = path.clone();
    lp.remove(0);
    return self.other_nodes[path[0]].get_node(lp);
  }
  fn readarr(json: Vec<u8>, brackloc: usize, jname: String, nindex: u32, indexed: bool) -> (JsonF, usize){
    let mut index = 0u32;
    let mut rdp = "".to_string();
    let mut rdn = 0f32;
    let mut rdb = false;
    let mut nt = NodeType::String;

    let mut brackop = false;
    let mut valgiv = false;

    let mut lnode = JsonF{ name: jname, strval: "".to_string(), numeral_val: 0.0, bolean: false, index: nindex, indexed: indexed, other_nodes: vec![], node_type: NodeType::Other };

    let mut i = brackloc;

    while json[i] != b']' {
      if !brackop{
        if (json[i] >= b'0' && json[i] <= b'9') || json[i] == b'.' || json[i] == b'-' || json[i] == b'e' || json[i] == b'E'{
          rdp += &(json[i] as char).to_string();
          nt = NodeType::Number;
          valgiv = true;
        }else if nt == NodeType::Number{
          rdn = rdp.parse().unwrap();
        }

        if json[i] == b't' && json[i+1] == b'r' && json[i+2] == b'u' && json[i+3] == b'e'{
          rdb = true;
          valgiv = true;
          nt = NodeType::Bolean;
          i+=3;
        }
        if json[i] == b'f' && json[i+1] == b'a' && json[i+2] == b'l' && json[i+3] == b's' && json[i+4] == b'e'{
          rdb = false;
          valgiv = true;
          i+=4;
          nt = NodeType::Bolean;
        }

        if json[i] == b'{' && brackloc != i{
          valgiv = false;
          let newval = Self::readbracket(json.clone(), i, "".to_string(), index, true);
          lnode.other_nodes.push(newval.0);
          i = newval.1;
          index += 1;
        }

        if json[i] == b'[' && brackloc != i{
          valgiv = false;
          let newval = Self::readarr(json.clone(), i, "".to_string(), index, true);
          lnode.other_nodes.push(newval.0);
          i = newval.1;
          index += 1;
        }

        if (json[i] == b'\n' || json[i] == b',') && valgiv{
          lnode.other_nodes.push(JsonF{ name: "".to_string(), strval: rdp.clone(), numeral_val: rdn, bolean: rdb, index: index.clone(), indexed: true, other_nodes: vec![], node_type: nt });
          index += 1;
          rdp = "".to_string();
          rdn = 0.0;
          rdb = false;
          nt = NodeType::String;
          valgiv = false;
        }
      }

      if json[i] == b'"'{
        brackop = !brackop;
        valgiv = true;
      }else{
        if brackop{
          rdp += &(json[i] as char).to_string();
        }
      }
      i += 1;
    }
    if valgiv {
      lnode.other_nodes.push(JsonF{ name: "".to_string(), strval: rdp.clone(), numeral_val: rdn, bolean: rdb, index: index.clone(), indexed: true, other_nodes: vec![], node_type: nt });
    }
    return (lnode, i);
  }
  fn readbracket(json: Vec<u8>, brackloc: usize, jname: String, nindex: u32, indexed: bool) -> (JsonF, usize){
    let mut name = "".to_string();
    let mut rdp = "".to_string();
    let mut rdn = 0f32;
    let mut rdb = false;
    let mut nt = NodeType::String;

    let mut brackop = false;
    let mut valgiv = false;

    let mut lnode = JsonF{ name: jname, strval: "".to_string(), numeral_val: 0.0, bolean: false, index: nindex, indexed: indexed, other_nodes: vec![], node_type: NodeType::Other };

    let mut i = brackloc;

    while json[i] != b'}' {
      if !brackop{
        if (json[i] >= b'0' && json[i] <= b'9') || json[i] == b'.' || json[i] == b'-' || json[i] == b'e' || json[i] == b'E'{
          rdp += &(json[i] as char).to_string();
          nt = NodeType::Number;
          valgiv = true;
        }else if nt == NodeType::Number{
          rdn = rdp.parse().unwrap();
        }

        if json[i] == b't' && json[i+1] == b'r' && json[i+2] == b'u' && json[i+3] == b'e'{
          rdb = true;
          valgiv = true;
          nt = NodeType::Bolean;
          i+=3;
        }
        if json[i] == b'f' && json[i+1] == b'a' && json[i+2] == b'l' && json[i+3] == b's' && json[i+4] == b'e'{
          rdb = false;
          valgiv = true;
          i+=4;
          nt = NodeType::Bolean;
        }

        if json[i] == b':'{
          name = rdp;
          valgiv = false;
          rdp = "".to_string();
        }

        if json[i] == b'{' && brackloc != i{
          let newval = Self::readbracket(json.clone(), i, name.clone(), 0, false);
          lnode.other_nodes.push(newval.0);
          i = newval.1;
          valgiv = false;
        }

        if json[i] == b'[' && brackloc != i{
          let newval = Self::readarr(json.clone(), i, name.clone(), 0, false);
          lnode.other_nodes.push(newval.0);
          i = newval.1;
          valgiv = false;
        }

        if (json[i] == b'\n' || json[i] == b',') && valgiv{
          lnode.other_nodes.push(JsonF{ name: name.clone(), strval: rdp.clone(), numeral_val: rdn, bolean: rdb, index: 0, indexed: false, other_nodes: vec![], node_type: nt });
          name = "".to_string();
          rdp = "".to_string();
          rdn = 0.0;
          rdb = false;
          nt = NodeType::String;
          valgiv = false;
        }
      }

      if json[i] == b'"'{
        brackop = !brackop;
        valgiv = true;
      }else{
        if brackop{
          rdp += &(json[i] as char).to_string();
        }
      }
      i += 1;
    }
    if valgiv{
      lnode.other_nodes.push(JsonF{ name: name.clone(), strval: rdp.clone(), numeral_val: rdn, bolean: rdb, index: 0, indexed: false, other_nodes: vec![], node_type: nt });
    }
    return (lnode, i);
  }
  pub fn from_text(json: &str) -> JsonF{
    let mut parsedjson = JsonF{ name: "".to_string(), strval: "".to_string(), numeral_val: 0.0, bolean: false, index: 0, indexed: false, other_nodes: vec![], node_type: NodeType::Other };
    let jsontext = json.as_bytes().to_vec();

    let mut i = 0;
    while i < jsontext.len(){
      if jsontext[i] == b'{'{
        let newval = Self::readbracket(jsontext.clone(), i, "".to_string(), 0, false);
        parsedjson = newval.0;
        i = newval.1;
      }
      i += 1;
    }
    return parsedjson;
  }
  pub fn printme(&mut self){
    if self.indexed{
      println!("index: {}", self.index);
    }else{
      println!("name: {}", self.name);
    }
    match self.node_type {
      NodeType::Bolean => {
        println!("  value: {}", self.bolean);
      },
      NodeType::String => {
        println!("  value: {}", self.strval);
      },
      NodeType::Number => {
        println!("  value: {}", self.numeral_val);
      },
      NodeType::Other => {
        println!("[");
        for i in 0..self.other_nodes.len(){
          self.other_nodes[i].printme();
          println!("");
        }
        println!("]");
      },
    }
  }
  pub fn load_from_file(path: &str) -> JsonF{
    let jsontext = fs::read(path).unwrap();
    JsonF::from_text(&String::from_utf8(jsontext).unwrap())
  }
}