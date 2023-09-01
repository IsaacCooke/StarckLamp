import 'package:flutter/material.dart';

import 'package:app/pages/home.dart';
import 'package:app/pages/settings.dart';

class Layout extends StatefulWidget {
  const Layout({super.key});

  @override
  State<Layout> createState() => LayoutState();
}

class LayoutState extends State<Layout> {
  int _selectedIndex = 0;

  static const List<Widget> _options = <Widget>[
    HomePage(),
    SettingsPage(),
  ];

  void _onItemTapped(int index){
    setState(() {
          _selectedIndex = index;
    });
  }

  @override
  Widget build(BuildContext context){
    return Scaffold(
      appBar: AppBar(
        title: const Text("Lamp App"), // TODO get a better name
      ),
      body: Center(
        child: _options.elementAt(_selectedIndex),
      ),
      bottomNavigationBar: BottomNavigationBar(
        items: const <BottomNavigationBarItem>[
          BottomNavigationBarItem(
            icon: Icon(Icons.home_sharp),
            label: "Home",
          ),
          BottomNavigationBarItem(
            icon: Icon(Icons.settings_sharp),
            label: "Settings"
          ),
        ],
        currentIndex: _selectedIndex,
        onTap: _onItemTapped,
      ),
    );
  }
}
