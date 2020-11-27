#![allow(dead_code)]
use Direction::*;
use Position::*;

enum Instruction {
  Direction(Direction),
  Newline,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Position {
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
}
impl Position {
  fn go(self, direction: Direction) -> Self {
    match (self, direction) {
      (One, Up) => One,
      (One, Down) => Four,
      (One, Left) => One,
      (One, Right) => Two,
      (Two, Up) => Two,
      (Two, Down) => Five,
      (Two, Left) => One,
      (Two, Right) => Three,
      (Three, Up) => Three,
      (Three, Down) => Six,
      (Three, Left) => Two,
      (Three, Right) => Three,
      (Four, Up) => One,
      (Four, Down) => Seven,
      (Four, Left) => Four,
      (Four, Right) => Five,
      (Five, Up) => Two,
      (Five, Down) => Eight,
      (Five, Left) => Four,
      (Five, Right) => Six,
      (Six, Up) => Three,
      (Six, Down) => Nine,
      (Six, Left) => Five,
      (Six, Right) => Six,
      (Seven, Up) => Four,
      (Seven, Down) => Seven,
      (Seven, Left) => Seven,
      (Seven, Right) => Eight,
      (Eight, Up) => Five,
      (Eight, Down) => Eight,
      (Eight, Left) => Seven,
      (Eight, Right) => Nine,
      (Nine, Up) => Six,
      (Nine, Down) => Nine,
      (Nine, Left) => Eight,
      (Nine, Right) => Nine,
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ComplexPosition {
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  A,
  B,
  C,
  D,
}
impl ComplexPosition {
  fn go(self, direction: Direction) -> Self {
    match (self, direction) {
      (ComplexPosition::One, Down) => ComplexPosition::Three,
      (ComplexPosition::Two, Down) => ComplexPosition::Six,
      (ComplexPosition::Two, Right) => ComplexPosition::Three,
      (ComplexPosition::Three, Up) => ComplexPosition::One,
      (ComplexPosition::Three, Down) => ComplexPosition::Seven,
      (ComplexPosition::Three, Left) => ComplexPosition::Two,
      (ComplexPosition::Three, Right) => ComplexPosition::Four,
      (ComplexPosition::Four, Down) => ComplexPosition::Eight,
      (ComplexPosition::Four, Left) => ComplexPosition::Three,
      (ComplexPosition::Five, Right) => ComplexPosition::Six,
      (ComplexPosition::Six, Up) => ComplexPosition::Two,
      (ComplexPosition::Six, Down) => ComplexPosition::A,
      (ComplexPosition::Six, Left) => ComplexPosition::Five,
      (ComplexPosition::Six, Right) => ComplexPosition::Seven,
      (ComplexPosition::Seven, Up) => ComplexPosition::Three,
      (ComplexPosition::Seven, Down) => ComplexPosition::B,
      (ComplexPosition::Seven, Left) => ComplexPosition::Six,
      (ComplexPosition::Seven, Right) => ComplexPosition::Eight,
      (ComplexPosition::Eight, Up) => ComplexPosition::Four,
      (ComplexPosition::Eight, Down) => ComplexPosition::C,
      (ComplexPosition::Eight, Left) => ComplexPosition::Seven,
      (ComplexPosition::Eight, Right) => ComplexPosition::Nine,
      (ComplexPosition::Nine, Left) => ComplexPosition::Eight,
      (ComplexPosition::A, Up) => ComplexPosition::Six,
      (ComplexPosition::A, Right) => ComplexPosition::B,
      (ComplexPosition::B, Up) => ComplexPosition::Seven,
      (ComplexPosition::B, Down) => ComplexPosition::D,
      (ComplexPosition::B, Left) => ComplexPosition::A,
      (ComplexPosition::B, Right) => ComplexPosition::C,
      (ComplexPosition::C, Up) => ComplexPosition::Eight,
      (ComplexPosition::C, Left) => ComplexPosition::B,
      (ComplexPosition::D, Up) => ComplexPosition::B,
      _ => self,
    }
  }
}

struct InstructionReader<'a> {
  instructions: &'a [u8],
}
impl<'a> InstructionReader<'a> {
  fn new(instructions: &'a str) -> Self {
    Self {
      instructions: instructions.as_bytes(),
    }
  }
}
impl<'a> Iterator for InstructionReader<'a> {
  type Item = Instruction;

  fn next(&mut self) -> Option<Self::Item> {
    let c = match self.instructions.get(0) {
      None => return None,
      Some(c) => c,
    };
    let instr = match c {
      b'U' => Instruction::Direction(Up),
      b'D' => Instruction::Direction(Down),
      b'L' => Instruction::Direction(Left),
      b'R' => Instruction::Direction(Right),
      b'\n' => Instruction::Newline,
      c => panic!("Got unexpected char in input: {:?}", c),
    };
    self.instructions = &self.instructions[1..];
    Some(instr)
  }
}

fn get_code(instructions: &str) -> Vec<Position> {
  let mut result = Vec::new();
  let mut position = Five;
  for instruction in InstructionReader::new(instructions) {
    match instruction {
      Instruction::Direction(d) => {
        position = position.go(d);
      }
      Instruction::Newline => {
        result.push(position);
      }
    }
  }
  result.push(position);
  result
}

fn get_complex_code(instructions: &str) -> Vec<ComplexPosition> {
  let mut result = Vec::new();
  let mut position = ComplexPosition::Five;
  for instruction in InstructionReader::new(instructions) {
    match instruction {
      Instruction::Direction(d) => {
        position = position.go(d);
      }
      Instruction::Newline => {
        result.push(position);
      }
    }
  }
  result.push(position);
  result
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn example_1() {
    let example = "ULL\n\
                  RRDDD\n\
                  LURDL\n\
                  UUUUD";
    assert_eq!(vec![One, Nine, Eight, Five], get_code(example));
  }

  const MY_INPUT: &'static str = "RLRDDRLLDLRLUDDULLDRUUULDDLRLUDDDLDRRDUDDDLLURDDDLDDDRDURUDRDRRULUUDUDDRRRLRRRRRLRULRLLRULDRUUDRLRRURDDRLRULDLDULLLRULURRUULLRLLDDDDLLDURRUDLDLURDRDRDLUUUDDRDUUDDULLUURRDRLDDULURRRUDLLULULDLLURURUDRRRRUDRLRDLRRLDDRDDLULDLLLURURDUDRRRRUULURLRDULDRLUDRRUDDUULDURUDLDDURRRDLULLUUDRLLDUUDLDRUDDRLLLLLLDUDUDDLRDLRRDRUDDRRRLLRRDLLRLDDURUURRRDDLDUULLDLDLRURDLLLDDRUUDRUDDDDULRLLDUULRUULLLULURRRLLULDLDUDLDLURUDUDULLDLLUUDRRDRLUURURURURDLURUUDLDRLUDDUUDULDULULLLDLDDULLULLDULRRDRULLURRRULLDDDULULURLRDURLLURUDDULLRUDLRURURRDRDUULDRUUDURDURDDLRDUUULDUUDRDURURDRRRURLLDDLLLURURULULUDLRDLDRDRURLRLULRDLU
UDLDURRULDRDDLDUULUDLDUULUURDDRUDRURRRUDRURLLDDRURLDLRDUUURDLLULURDDUDDDRRRURLLDLDLULRDULRLULDLUUDLLRLDLRUUULDDUURDLDDRRDLURLDUDDRURDRRURDURRRLUULURDDLRDLDRRRLDUDRLRLLRLDDUULDURUUULLLRRRRRRRDRRRDRLUULDLDDLULDRDUDLLUDRRUDRUUDULRLUURDDDDRRUUDLURULLLURDULUURDRDDURULRUDRRDLRDUUUUUDDDRDRDDRUDRDDDRLRUUDRDRDDDLUDRDRLDRDDRULURDRLDRUDUDRUULRLLUDRDRLLLLDUDRRLLURDLLLDRRUDDUDRLRLDUDRLURRUUULURDDRUURRLDRLRRRUUDLULDDDRDLDUUURLLUULDDRRUDLDDRUDUDUURURDDRDULLLLLULRRRDLRRRDDDLURDDDDLUULLLRDDURRRRLURRLDDLRUULULRDRDDDDLDUUUUUUDRRULUUUDD
UURDRRUDLURRDDDLUDLRDURUDURDLLLLRDLRLRDDRDRDUUULRDLLDLULULRDUDDRRUUDURULDLUDLRDRUDLDDULLLDDRDLLDULLLURLLRDDLDRDULRRDDULRDURLLRUDRLRRLUDURLDRDLDLRLLLURLRRURDLDURDLUDULRDULLLDRDDRDLDRDULUULURDRRRLDRRUULULLDDRRLDLRUURLRUURLURRLLULUUULRLLDDUDDLRLDUURURUDLRDLURRLLURUDLDLLUDDUULUUUDDDURDLRRDDDLDRUDRLRURUUDULDDLUUDDULLDDRRDDRRRUDUDUDLDLURLDRDLLLLDURDURLRLLLUUDLRRRRUDUDDLDLRUURRLRRLUURRLUDUDRRRRRRRLDUDDRUDDLUDLRDDDRLDUULDRDRRDLDRURDLDRULRLRLUDRDLRRUURUUUUDLDUUULLLRRRRRDLRRURDDLLLLUULDLLRULLUDLLDLLUDLRLRRLRURDDRRL
URDRDLLRDDDLLLDDLURLRURUURRRLUURURDURRLLUDURRLRLDLUURDLULRRDRUDDLULDLDRLDLRLRRLLLDDDUDDDLRURURRLLDRRRURUDLRDDLLDULDDLDRLUUUDRRRULDUULRDDDLRRLLURDDURLULRDUDURRLLDLLRLDUDDRRDDLRLLLDUDRLUURRLLDULRLDLUUUUUDULUDLULUDDUURRURLDLDRRLDLRRUDUDRRDLDUDDLULLDLLRDRURDRDRRLDDDDRDDRLLDDDLLUDRURLURDRRRRRUDDDUDUDDRDUUDRRUDUDRLULDDURULUURUUUURDRULRLRULLDDRRRUULRRRRURUDLDLRDLLDRLURLRUULLURDUDULRRURLRLLRRLLLURULRRRLDDUULLUUULRRDRULUUUUDRDRRDLRURLRLLRLRRRDRDRLDLUURUURULLDLULRRLRRDRULRRLLLDDURULLDLDLDLUUURDLDLUUDULRLLUDDRRDLLDLDLDURLUURRDDRRURDRLUDRLUUUDLDULDLUDRLDUDDLLRUDULLLLLDRRLLUULLUUURRDDUURDLLRDDLRLLU
LDUDRRDLUUDDRLLUUULURLDUDLUDLRLDRURLULRLLDDLRRUUUDDDDRDULDDUUDLRUULDRULLRDRUDDURLDUUURRUDUDRDRDURRDLURRRDRLDLRRRLLLRLURUURRDLLRDLDDLLRDUDDRDUULRULRRURLUDDUDDDUULLUURDULDULLLLRUUUDDRRRLDDDLDLRRDRDRDLUULRLULDRULDLRDRRUDULUDLLUDUULRDLRRUUDDLLDUDDRULURRLULDLDRRULDDRUUDDLURDLRDRLULRRLURRULDUURDLUDLLDRLDULLULDLLRDRDLLLUDLRULLRLDRDDDLDDDLRULDLULLRUUURRLLDUURRLRLDUUULDUURDURRULULRUUURULLLRULLURDDLDRLLRDULLUDLDRRRLLLLDUULRRLDURDURDULULDUURLDUDRLRURRDLUUULURRUDRUUUDRUR";

  #[test]
  fn test_my_input() {
    assert_eq!(vec![One, Eight, Eight, Four, Three], get_code(MY_INPUT));
  }

  #[test]
  fn part_two_example_1() {
    let example = "ULL\n\
                  RRDDD\n\
                  LURDL\n\
                  UUUUD";
    assert_eq!(
      vec![
        ComplexPosition::Five,
        ComplexPosition::D,
        ComplexPosition::B,
        ComplexPosition::Three
      ],
      get_complex_code(example)
    );
  }

  #[test]
  fn part_two_my_input() {
    assert_eq!(
      vec![
        ComplexPosition::Six,
        ComplexPosition::Seven,
        ComplexPosition::B,
        ComplexPosition::B,
        ComplexPosition::Nine
      ],
      get_complex_code(MY_INPUT)
    );
  }
}
