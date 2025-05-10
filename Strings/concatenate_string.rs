fn concatenate_strings(s1: &str, s2: &str) -> String{
    s1.to_string() + s2
}

fn main(){
  let str_result = concatenate_strings("This is a", " concatenated string"); 
    
  println!("{}", str_result);
}
