import 'package:flutter/material.dart';

import 'package:graphql_flutter/graphql_flutter.dart';

import 'package:app/data/services/graphql_client.dart';
import 'package:app/pages/layout.dart';
import 'package:app/themes/color_schemes.g.dart';

void main(){
  runApp(const App());
}

final navigatorKey = GlobalKey<NavigatorState>();

class App extends StatelessWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context){
    return GraphQLProvider(
      client: client,
      child: MaterialApp(
        debugShowCheckedModeBanner: false,
        title: "Lamp App", // TODO change this
        home: const Layout(),
        theme: ThemeData(useMaterial3: true, colorScheme: lightColorScheme),
        darkTheme: ThemeData(useMaterial3: true, colorScheme: darkColorScheme),
        navigatorKey: navigatorKey,
      ),
    );
  }
}
