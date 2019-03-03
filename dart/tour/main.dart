import 'package:dio/dio.dart';

main() async {
  Response response;
response=await dio.get("http://www.freshfresh.com")
print(response.data.toString());
}
