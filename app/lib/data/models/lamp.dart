class Lamp {
  final int? id;
  final String name;
  final String description;
  final int red; 
  final int green;
  final int blue;
  final bool isOn;
  final int? userId;

  Lamp({
    this.id,
    required this.name,
    required this.description,
    required this.red,
    required this.green,
    required this.blue,
    required this.isOn,
    this.userId,
  });

  factory Lamp.fromJson(Map<String, dynamic> json) => Lamp(
    id: json['id'],
    name: json['name'],
    description: json['description'],
    red: json['red'],
    green: json['green'],
    blue: json['blue'],
    isOn: json['is_on'],
    userId: json['user_id'],
  );
}
