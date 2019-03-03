main() {
  var a = new List<P>();

  a.add(new Person());
  a.add(new Person1());

  print(a);
}

class P {

}

class Person extends P {
  static var a = 100;
  static sayStatic(){
    print("say static");
  }

  String toString() {
    return "hello";
  }
}

class Person1 extends P{
  String toString() {
    return "world";
  }
}
