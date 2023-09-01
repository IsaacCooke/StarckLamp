import 'dart:convert';

class User {
  // id, username, email, password
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
}
