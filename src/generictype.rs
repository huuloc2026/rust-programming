fn main(){
    let point = Point {
        x: 16f64,
        y: 16f64,
    };
    let new_point_gen = PointGen {
        x: 16f32,
        y: 16f32,
    };
    let new_point_gen2 = PointGenVari {
        x: 16f64,
        y: 16f32,
    };
}
pub struct Point {
    x: f64,
    y: f64,
}
pub struct PointGen<T> {
    x: T,
    y: T,
}
// 2 generic type
pub struct PointGenVari<T,U> {
    x: T,
    y: U,
}

fn main(){
    let p1 = Pointer { x : 9,y : 5};
    let p2 = Pointer { x : 10.5,y : 5.5};
    let p3 = Pointer { x : String::from("Jake"),y : 'c'};
      let p4 = p1.mixed(p2);
      println!("{},{}",p4.x,p4.y)
  }
  struct Pointer<T,U> {
      x: T,
      y: U,
  
  }
  
  impl<T,U> Pointer<T,U> {
      fn mixed<V,W>(self,other:Pointer<V,W>) -> Pointer<T,W> {
          Pointer {
              x: self.x,
              y: other.y,
          }
      }
  }