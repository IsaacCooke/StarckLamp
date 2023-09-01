import 'package:flutter/material.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => HomeState();
}

class HomeState extends State<HomePage> {

  @override
  Widget build(BuildContext context){
    return const Text("Home Page");
  }
}
