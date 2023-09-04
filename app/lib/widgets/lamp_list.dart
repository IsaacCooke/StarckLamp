import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';

class LampList extends StatelessWidget {
  const LampList({super.key});

  const _gqlDocument = """
    query {
      lampsByUser(\$userId: Int) {
        id
        name
        description
        red
        green
        blue
        isOn
      }
    }
    """

  @override
  Widget build(BuildContext context){
    Query(
       options: QueryOptions(

      ), 
    )
  }
}
