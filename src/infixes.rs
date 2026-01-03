pub fn infix_binding_power(op: char) -> Option<(u8, u8)> {
   let res = match op {
      '?'       => (1, 2),
      ':'       => (3, 4),
      '='       => (5, 6),
      '+' | '-' => (7, 8),
      '*' | '/' => (9, 10),
      _ => return None,
   };

   Some(res)
}

pub fn prefix_binding_power(op: char) -> ((), u8) {
   match op {
      '+' | '-' => ((), 5),
      _ => panic!("BAD TOKEN: {:?}", op),
   }
}

pub fn postfix_binding_power(op: char) -> Option<(u8, ())> {
   let res = match op {
      '!' | '[' => (7, ()),
      _ => return None
   };

   Some(res)
}