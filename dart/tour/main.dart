import 'package:dio/dio.dart';

main() async {
  Dio dio = new Dio();
  Response response;
  response = await dio.get("http://www.freshfresh.com");
  print(response.data.toString());
  response = await dio.get("http://wx.freshfresh.com");
  print(response.data.toString());

  var result = await Future.delayed(new Duration(seconds: 1), () {
    return "OK";
  });
  print(result);
}
