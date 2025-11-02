use std::fs;

pub struct JsonF{
  name: String,
  strval: String,
  numeral_val: f32,
  strvalar: Vec<String>,
  numeral_valar: Vec<f32>,
  other_param: Vec<JsonF>,
}

impl JsonF {
  pub fn get_node(&mut self, path: Vec<usize>) -> &mut JsonF{
    if path.len() < 1{
      return self;
    }
    let mut lp = path.clone();
    lp.remove(0);
    return self.other_param[path[0]].get_node(lp);
  }
  pub fn from_text(json: &str) -> JsonF{
    let mut parsedjson = JsonF{ name: "".to_string(), strval: "".to_string(), numeral_val: 0.0, strvalar: vec![], numeral_valar: vec![], other_param: vec![] };
    let jsontext = json.as_bytes().to_vec();

    let mut jsfr = 0usize;
    let mut brakeop = false;
    let mut stringvl = "".to_string();
    let mut stringvlar: Vec<String> = vec![];
    let mut numvlar: Vec<f32> = vec![];
    let mut entrar = vec![];
    let mut valgiv = false;
    let mut txtarg = false;
    let mut numarg = false;
    let mut arrwr = false;
    let mut backstep = false;
    let mut strrdst = false;

    while jsfr < jsontext.len(){
      if jsontext[jsfr] == b'{' && valgiv{
        valgiv = false;
      }
      if jsontext[jsfr] == b'}' && entrar.len() > 1{
        entrar.pop();
        let enln = entrar.len();
        entrar[enln-1]+=1;
        valgiv = false;
      }
      if jsontext[jsfr] == b'[' && valgiv{
        arrwr = true;
        txtarg = false;
        numarg = false;
      }
      if jsontext[jsfr] == b']' && valgiv{
        if txtarg {
          //for i in 0..stringvlar.len(){
          //  println!("{}: {}", i, stringvlar[i]);
          //}
          parsedjson.get_node(entrar.clone()).strvalar = stringvlar.clone();
        }
        if numarg {
          //for i in 0..numvlar.len(){
          //  println!("{}: {}", i, numvlar[i]);
          //}
          parsedjson.get_node(entrar.clone()).numeral_valar = numvlar.clone();
        }
        stringvlar = vec![];
        numvlar = vec![];
        valgiv = false;
        txtarg = false;
        numarg = false;
        arrwr = false;
        backstep = true;
      }
      if jsontext[jsfr] == b'"'{
        brakeop = !brakeop;
        txtarg = true;
        strrdst = true;
      }
      if (jsontext[jsfr] == b'0' || jsontext[jsfr] == b'1' || jsontext[jsfr] == b'2' || jsontext[jsfr] == b'3' ||  jsontext[jsfr] == b'4' || jsontext[jsfr] == b'5' || jsontext[jsfr] == b'6' || jsontext[jsfr] == b'7' ||  jsontext[jsfr] == b'8' || jsontext[jsfr] == b'9' || jsontext[jsfr] == b'.') && !txtarg{
        stringvl += &(jsontext[jsfr] as char).to_string();
        numarg = true;
        strrdst = true;
      }
      if brakeop && jsontext[jsfr] != b'"'{
        stringvl += &(jsontext[jsfr] as char).to_string();
      }
      if valgiv && (jsontext[jsfr] == b',' || jsontext[jsfr] == b'\n'){
        if txtarg && !arrwr{
          parsedjson.get_node(entrar.clone()).strval = stringvl.clone();
          stringvl = "".to_string();
          backstep = true;
          valgiv = false;
          txtarg = false;
        }
        if numarg && !arrwr{
          parsedjson.get_node(entrar.clone()).numeral_val = stringvl.parse().unwrap();
          stringvl = "".to_string();
          backstep = true;
          valgiv = false;
          numarg = false;
        }
        if txtarg && arrwr && strrdst{
          stringvlar.push(stringvl.clone());
          stringvl = "".to_string();
          backstep = true;
          strrdst = false;
        }
        if numarg && arrwr && strrdst{
          numvlar.push(stringvl.parse().unwrap());
          stringvl = "".to_string();
          backstep = true;
          strrdst = false;
        }
      }
      if jsontext[jsfr] == b':' && !arrwr && !valgiv{
        let mut lentr = entrar.clone();
        if backstep{
          lentr.pop();
        }
        parsedjson.get_node(lentr).other_param.push(JsonF{ name: stringvl.clone(), strval: "".to_string(), numeral_val: 0.0, strvalar: vec![], numeral_valar: vec![], other_param: vec![] });
        if backstep{
          let enln = entrar.len();
          entrar[enln-1]+=1;
        }else{
          entrar.push(0usize);
        }
        stringvl = "".to_string();
        txtarg = false;
        numarg = false;
        arrwr = false;
        valgiv = true;
        backstep = false;
      }
      jsfr += 1;
    }
    return parsedjson;
  }
  pub fn printme(&mut self){
    println!("name: {}", self.name);
    println!("  string_value: {}", self.strval);
    println!("  Numeral_value: {}", self.numeral_val);
    println!("  Numeral_value_arr_len: {}", self.numeral_valar.len());
    println!("  String_value_arr_len: {}", self.strvalar.len());
    print!("  Other_json_value_array: [");
    for i in 0..self.other_param.len(){
      println!("");
      self.other_param[i].printme();
    }
    println!("]");
  }
  pub fn load_from_file(path: &str) -> JsonF{
    let jsontext = fs::read(path).unwrap();
    JsonF::from_text(&String::from_utf8(jsontext).unwrap())
  }
}