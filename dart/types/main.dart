main() {
  var user = User("haha", 2);
  printUser(user);
  printUser(1);
}

printUser(user) {
  if (user is User) {
    print(user.age);
  } else {
    print(user);
  }
}

class User {
  String name;
  int age;

  User(name, age)
      : name = name,
        age = age;
}
