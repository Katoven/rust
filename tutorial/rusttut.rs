use std::{
    i8,
    i16,
    i32,
    i64,
    u8,
    u32,
    u64,
    isize,
    usize,
    f32,
    f64
};

use std::io::stdin;

fn main() {
  println!("Hello world!");

  let _num = 10;

  let mut _age: i32 = 40;

  println!("Max i8 {}", i8::MAX);
  println!("Min i8 {}", i8::MIN);
  println!("Max i16 {}", i16::MAX);
  println!("Min i16 {}", i16::MIN);
  println!("Max i16 {}", i32::MAX);
  println!("Min i16 {}", i32::MIN);
  println!("Max i64 {}", i64::MAX);
  println!("Min i64 {}", i64::MIN);
  println!("Max u8 {}", u8::MAX);
  println!("Max u32 {}", u32::MAX);
  println!("Max u64 {}", u64::MAX);
  println!("Max isize {}", isize::MAX);
  println!("Min isize {}", isize::MIN);
  println!("Max usize {}", usize::MAX);
  println!("Min usize {}", usize::MIN);
  println!("Max f32 {}", f32::MAX);
  println!("Min f32 {}", f32::MIN);
  println!("Max f64 {}", f64::MAX);
  println!("Min f64 {}", f64::MIN);

  let _is_it_true: bool = true;

  let _let_x: char = 'x';

  println!("I am {} years old", _age );

  let (_f_name, _m_name, _l_name) = ("Marco", "van", "Kaathoven");

  println!("Is it {0} that {1} is {0}", _is_it_true, _let_x);

  println!("{:.2}", 1.234);

  println!("B: {:b} H: {:x} O: {:o}", 10, 10, 10);

  println!("{ten:>ws$}", ten=10, ws=5);
  println!("{ten:>0ws$}", ten=10, ws=5);

  println!("5 + 4 = {}", 5 + 4);
  println!("5 - 4 = {}", 5 - 4);
  println!("5 * 4 = {}", 5 * 4);
  println!("5 / 4 = {}", 5 / 4);
  println!("5 % 4 = {}", 5 % 4);

  let mut _neg_4 = -4i32;

  println!("abs(-4) = {}", _neg_4.abs());
  println!("4 ^ 6 = {}", 4i32.pow(6));
  println!("sqrt 9 = {}", 9f64.sqrt());
  println!("cbrt 9 = {}", 9f64.cbrt());
  println!("round 1.45 = {}", 1.45f64.round());
  println!("floor 1.45 = {}", 1.45f64.floor());
  println!("ceil 1.45 = {}", 1.45f64.ceil());
  println!("e ^ 2 = {}", 2f64.exp());
  println!("log(2) = {}", 2f64.ln());
  println!("log10(2) = {}", 2f64.log10());
  println!("90 to radians = {}", 90f64.to_radians());
  println!("PI to degrees = {}", 3.14f64.to_degrees());
  println!("Max 4, 5 = {}", 4f64.max(5f64));
  println!("Min 4, 5 = {}", 4f64.min(5f64));
  println!("Sin 3.14 = {}", 3.14f64.sin());
  println!("Cos 3.14 = {}", 3.14f64.cos());
  println!("Tan 3.14 = {}", 3.14f64.tan());

  println!("What is your age?");

  let mut _input_text = String::new();
  stdin().read_line(&mut _input_text).expect("Failed to read age");
  let _trimmed = _input_text.trim();
  let mut _age_old = 0;
  match _trimmed.parse::<u32>() {
      Ok(i) => _age_old = i,
      Err(..) => println!("this was not an integer: {}", _trimmed),
  };

  println!("Your are {} years old", _age_old);
  if _age_old == 5 {
      println!("Go to kindergarten");
  }else if _age_old > 5 && _age_old <= 18 {
       println!("Go to grade {}", _age_old-5);
  }else if _age_old > 18 && _age_old <= 25 {
       println!("Go to College");
  }else {
       println!("Do what you want");
  }

  let _can_cote = if _age_old>=18 {true} else {false};
  println!("Can vote : {}", _can_cote);

  let mut _x = 1;

  loop {
      if (_x % 2) == 0 {
          println!("x = {}", _x);

          _x += 1;
          continue;
      }
      if _x > 10 {
          break;
      }
      _x += 1;
  }

  let mut _y = 1;

  while _y <= 10 {
      println!("y = {}", _y);
      _y += 1;
  }

  for _z in 1..10 {
      println!("z = {}", _z);
  }

  let _rand_string = "I am a random string";

  println!("Length = {}", _rand_string.len());

  let (_first, _second) = _rand_string.split_at(6);
  println!("First = {} Second = {}", _first, _second);

  let _count = _rand_string.chars().count();
  let mut _chars = _rand_string.chars();

  let mut _indiv_char = _chars.next();

  loop {
      match _indiv_char {
          Some(x) => println!("{}", x),
          None => break,
      }
      _indiv_char = _chars.next();
  }

  let mut _iter = _rand_string.split_whitespace();

  let mut _indiv_word = _iter.next();

  loop {
      match _indiv_word {
          Some(x) => println!("{}", x),
          None => break,
      }
      _indiv_word = _iter.next();
  }

  let _rand_string2 = "I am a random string\nThere are other strings like this\nThis string is the best";
  let mut _lines = _rand_string2.lines();
  let mut _indiv_line = _lines.next();
  loop {
      match _indiv_line {
          Some(x) => println!("{}", x),
          None => break,
      }
      _indiv_line = _lines.next();
  }

  println!("Find best: {}", _rand_string2.contains("best"));

  'outer: loop {
      let _number = 10;
      println!("Pick a Number");

      loop{
          let mut _line = String::new();
          let _input = stdin().read_line(&mut _line);

          let _guess: Option<i32> = _input.ok().map_or(None, |_| _line.trim().parse().ok());

          match _guess {
              None => println!("Enter a number"),
              Some(n) if n == _number => {
                  println!("You Guessed it");
                  break 'outer;
              }
              Some(n) if n < _number => println!("Too Low"),
              Some(n) if n > _number => println!("Too High"),
              Some(_) => println!("Error")
          }
      }
  }

  let _rand_array = [1, 2, 3];

  println!("{}", _rand_array[0]);
  println!("{}", _rand_array.len());
  println!("Second 2 : {:?}", &_rand_array[1..3]);

  let mut _vect1 = vec![1,2,3,4,5];
  println!("Item2 {}", _vect1[1]);

  for i in &_vect1 {
      println!("Vect: {}", i);
  }

  _vect1.push(6);

  for i in &_vect1 {
      println!("Vect: {}", i);
  }

  _vect1.pop();

  for i in &_vect1 {
      println!("Vect: {}", i);
  }

  let _rand_tuple = ( "Marco", 39 );
  let _rand_tuple2: (&str, i8) = ( "Marco", 39 );

  println!("Name : {}", _rand_tuple2.0);

  say_hello("Grace");

  println!("5 + 4 = {}", get_sum(5, 4));

  let sum = get_sum;
  println!("6 + 4 = {}", sum(6, 4));

  let sum_nums = |x: i32, y: i32| x + y;
  println!("7 + 8 = {}", sum_nums(7, 8));

  let _num_ten = 10;
  let add_10 = |x: i32| x + _num_ten;

  println!("5 + 10 = {}", add_10(5));

  let _vect2 = _vect1;
  //println!("vect1[0] = {}", _vect1[0]);

  let _prim_val = 1;
  let _prim_val2 = _prim_val;

  println!("prim_val : {}", _prim_val);

  println!("Sum of Vects : {}", sum_vects(&_vect2));

  println!("Vect : {:?}", _vect2);

  let mut _circle1 = Circle {
      x: 10.0, y: 10.0, radius: 10.0
  };

  println!("X : {} Y : {} R : {}", _circle1.x, _circle1.y, _circle1.radius);

  println!("Circle Radius : {}", get_radius(&_circle1));

  println!("Circle X : {}", _circle1.get_x());

  println!("Circle Area : {}", _circle1.area());

  let mut _rect1 = Rectangle {
      height: 10.0, width: 10.0
  };

  println!("Rectangle Area : {}", _rect1.area());

  let _hulk = Hero::Strong(100);
  let _quicksilver = Hero::Fast;
  let _spiderman = Hero::Info {name: "Spiderman".to_owned(), secret: "Peter Parker".to_owned()};

  get_info(_hulk);
  get_info(_quicksilver);
  get_info(_spiderman);
}

fn say_hello(name: &str) {
    println!("Hello {}", name);
}

fn get_sum(num1: i32, num2: i32) -> i32{
    num1 + num2
}

fn sum_vects(v1: &Vec<i32>) -> i32{
    let sum = v1.iter().fold(0, |mut sum, &x| {sum += x; sum});
    return sum;
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

fn get_radius(circle: &Circle) -> f64 {
    circle.radius
}

impl Circle {
    pub fn get_x(&self) -> f64 {
        self.x
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        3.14159 * (self.radius * self.radius)
    }
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }
}

enum Hero {
    Fast,
    Strong(i32),
    Info {name: String, secret: String}
}

fn get_info(h: Hero){
    match h {
        Hero::Fast => println!("Fast"),
        Hero::Strong(i) => println!("Lifts {} tons", i),
        Hero::Info {name, secret} => {
            println!("{} is {}", name, secret);
        },
    }
}
