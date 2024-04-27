//
// fn is_allowed(request: &HttpRequest) -> bool {
//     let authorization = request.headers().get("Authorization");
//     let authorization = authorization.unwrap().to_str();
//     let decoded = general_purpose::STANDARD.decode(authorization).unwrap();
//     let decoded = String::from_utf8(decoded).unwrap();
//     let decoded = decoded.split(":").collect::<Vec<&str>>();
//     let username = decoded[0];
//     let password = decoded[1];
//     request.app_data::<RoadworkServerData>()
//         .unwrap().user_repository.is_user_valid(username, password)
// }