import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';

class LampList extends StatelessWidget {
  const LampList({super.key});

  @override
  Widget build(BuildContext context){
    return GraphQLProvider(
      client
    )
  }
}
