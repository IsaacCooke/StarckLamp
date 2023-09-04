import 'dart:convert';

class User {
  final int? id;
  final String username;
  final String email;
  final String password;

  const User({
    this.id,
    required this.username,
    required this.email,
    required this.password,
  });
  
  factory User.fromJson(Map<String, dynamic> json) => User(
    id: json['id'],
    username: json['username'],
    email: json['email'],
    password: json['password'],
  );
}

class UserList {
  final List<User> users;

  UserList({required this.users});

  factory UserList.fromJson(Map<String, dynamic> json) => UserList(
    users: (json['data'] as List<dynamic>)
    .map((userJson) => User.fromJson(userJson))
    .toList(),
  );
}
