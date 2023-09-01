import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';

class LampList extends StatelessWidget {
  const LampList({super.key});

  const _gqlDocument = """
    query Lamp
  """

  @override
  Widget build(BuildContext context){
    Query(
       options: QueryOptions(

      ), 
    )
  }
}
