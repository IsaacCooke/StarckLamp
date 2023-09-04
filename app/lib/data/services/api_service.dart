import 'package:graphql_flutter/graphql_flutter.dart';

import 'package:app/data/models/lamp.dart';
import 'package:app/data/services/graphql_client.dart';

class ApiService {
  final GraphQlClient _client = client.value;

  Future<List<Lamp>> getLampsByUser(int userId) async {
    final QueryOptions options = QueryOptions(
      document: gql(r'''
        query {
          lampsByUser($userId: Int!) {
            id
            name
            description
            red
            green
            blue
            isOn
            userId
          }
        }
      '''),
    variables: {'userId': userId},
    );

    final QueryResult result = await _client.query(options);

    if (result.hasException) {
      throw result.exception;
    }

    final List<dynamic> lampsData = result.data?['lampsByUser'];

    print(lampsData);

    return lampsData.map((lamp) => Lamp.fromJson(lamp)).toList();
  }
}
